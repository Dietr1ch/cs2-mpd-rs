use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use clap::Parser;
use color_eyre::eyre::WrapErr;

use cs2_mpd_rs::gamestate::GameData;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
	#[arg(long, env = "LISTEN_ADDRESS", default_value = "127.0.0.1:3000")]
	pub listen_address: String,
	#[arg(long, env = "MPD_ADDRESS", default_value = "127.0.0.1:6600")]
	pub mpd_address: String,
	#[arg(long, env = "STEAM_ID")]
	pub steam_id: String,

	#[arg(
		long,
		env = "GAME_PATH",
		default_value = "~/.local/share/Steam/steamapps/common/Counter-Strike Global Offensive/"
	)]
	pub game_path: PathBuf,
}

#[derive(Debug)]
struct MusicPlayer {
	address: String,
	mpd: mpd::Client,
	last_state: Option<mpd::State>,
}

impl MusicPlayer {
	fn reset(&mut self) -> Result<(), mpd::error::Error> {
		tracing::info!("Reconnecting to MPD...");
		self.last_state = None;
		self.mpd = mpd::Client::connect(&self.address)?;
		self.last_state = Some(self.mpd.status()?.state);
		tracing::info!("Current state: {:?}", self.last_state);
		Ok(())
	}

	fn set_state(&mut self, state: mpd::State) -> Result<(), mpd::error::Error> {
		if let Some(last_state) = &self.last_state
			&& last_state == &state
		{
			return Ok(());
		}

		let status = cs2_mpd_rs::music::set_mpd(&mut self.mpd, state)?;
		self.last_state = Some(status.state);
		Ok(())
	}
}

#[derive(Clone, Debug)]
struct AppState {
	music_player: Arc<Mutex<MusicPlayer>>,
	steam_id: String,
}

impl AppState {
	fn desired_state(&self, game_data: &GameData) -> mpd::State {
		use cs2_mpd_rs::gamestate::RoundPhase;

		if let Some(round) = &game_data.round
			// Round live
			&& round.phase == RoundPhase::Live
			// Active player
			&& let Some(player) = &game_data.player
			&& player.steam_id == self.steam_id
			&& let Some(state) = &player.state
			// Alive
			&& state.health > 0
		{
			mpd::State::Pause
		} else {
			mpd::State::Play
		}
	}

	fn set_music(&self, music_state: mpd::State) -> Result<(), mpd::error::Error> {
		let mut music_player = self.music_player.lock().unwrap();
		match music_player.set_state(music_state) {
			Ok(_) => Ok(()),
			Err(mpd::error::Error::Io(io_err)) => {
				tracing::error!("MPD call failed with IO error: {io_err:?}");
				music_player.reset()
			}
			Err(e) => {
				tracing::error!("MPD call failed: {e:?}");
				Err(e)
			}
		}
	}

	fn play_or_pause(&self, game_data: &GameData) {
		match &game_data.player {
			Some(player) => {
				if player.steam_id.as_str() != self.steam_id {
					tracing::info!(
						"Who's {} ({})? Are you spectating?",
						player.name,
						player.steam_id
					);
					return;
				}
			}
			None => {
				return;
			}
		}

		tracing::debug!("Game data:\n{game_data:?}");
		let desired_music_state = self.desired_state(game_data);
		tracing::debug!("Desired music state: {desired_music_state:?}");
		if let Err(e) = self.set_music(desired_music_state) {
			tracing::error!("Couldn't change MPD state; {e:?}");
		}
	}
}

#[actix_web::post("/")]
async fn cs2_event(app_state: web::Data<AppState>, game_data: web::Json<GameData>) -> String {
	tracing::info!("model: {:?}", &game_data);
	app_state.play_or_pause(&game_data);

	format!("GameData: {game_data:?}\n\nAppState: {app_state:?}!")
}

// Sample JSON error handler
// - https://github.com/actix/examples/blob/master/json/json-decode-error/src/main.rs
fn json_error_handler(
	err: actix_web::error::JsonPayloadError,
	_req: &actix_web::HttpRequest,
) -> actix_web::error::Error {
	use actix_web::HttpResponse;
	use actix_web::error::JsonPayloadError;

	let error_message = format!("Bad JSON payload: {err}");
	let resp = match &err {
		JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().body(error_message),
		JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
			HttpResponse::UnprocessableEntity().body(error_message)
		}
		_ => HttpResponse::BadRequest().body(error_message),
	};

	actix_web::error::InternalError::from_response(err, resp).into()
}

#[actix_web::main]
async fn main() -> color_eyre::eyre::Result<()> {
	color_eyre::install()?;
	tracing_subscriber::fmt::init();

	let args = Args::parse();
	tracing::trace!("Args: {args:?}");

	let state = AppState {
		music_player: Arc::new(Mutex::new(MusicPlayer {
			address: args.mpd_address.to_string(),
			mpd: mpd::Client::connect(&args.mpd_address).wrap_err(format!(
				"Couldn't connect to MPD server at {address}",
				address = &args.mpd_address,
			))?,
			last_state: None,
		})),
		steam_id: args.steam_id,
	};

	let mut cs2_config_path = args.game_path.clone();
	cs2_config_path.push("game/csgo/cfg/gamestate_integration_cs2-mpd.cfg");
	tracing::info!("Current config path: {cs2_config_path:?}");
	let cs2_config = std::fs::read_to_string(cs2_config_path)?;
	tracing::trace!("Current config:\n{cs2_config}");

	HttpServer::new(move || {
		App::new()
			.wrap(actix_web::middleware::Logger::new(
				"%a '%r' %s %b '%{Referer}i' '%{User-Agent}i' %T",
			))
			.app_data(web::Data::new(state.clone()))
			// custom `Json` extractor configuration
			.app_data(
				web::JsonConfig::default()
					// register error_handler for JSON extractors.
					.error_handler(json_error_handler),
			)
			.service(cs2_event)
	})
	.workers(2)
	.bind(args.listen_address)?
	.run()
	.await?;

	Ok(())
}
