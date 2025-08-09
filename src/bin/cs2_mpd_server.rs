use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use color_eyre::eyre::WrapErr;

use cs2_mpd_rs::gamestate::GameData;

use clap::Parser;

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

#[derive(Clone, Debug)]
struct AppState {
    mpd: Arc<Mutex<mpd::Client>>,
    steam_id: String,
}

impl AppState {
    fn try_play_or_pause(&self, game_data: &GameData) -> Result<(), mpd::error::Error> {
        use cs2_mpd_rs::music::MpdState;

        let music_state = match game_data.round.phase.as_ref() {
            "freezetime" | "warmup" => MpdState::Play,
            _ => {
                if game_data.player.state.health > 0 {
                    // Still alive
                    MpdState::Pause
                } else {
                    // No longer alive
                    MpdState::Play
                }
            }
        };

        let mut mpd = self.mpd.lock().unwrap();
        cs2_mpd_rs::music::set_mpd(&mut mpd, music_state)?;

        Ok(())
    }

    fn play_or_pause(&self, game_data: &GameData) {
        if game_data.player.steamid.as_str() != self.steam_id {
            tracing::info!("Who's {:?}?", game_data.player.steamid);
            return;
        }

        tracing::debug!("Game data:\n{game_data:?}");
        match self.try_play_or_pause(game_data) {
            Ok(_) => {}
            _ => {
                tracing::error!("Oh, no.");
            }
        }
    }
}

#[actix_web::post("/")]
async fn index(app_state: web::Data<AppState>, game_data: web::Json<GameData>) -> String {
    tracing::info!("model: {:?}", &game_data);
    app_state.play_or_pause(&game_data);

    format!("GameData: {game_data:?}\n\nAppState: {app_state:?}!")
}

#[actix_web::main]
async fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    tracing::trace!("Args: {args:?}");

    let state = AppState {
        mpd: Arc::new(Mutex::new(
            mpd::Client::connect(&args.mpd_address).wrap_err(format!(
                "Couldn't connect to MPD server at {address}",
                address = &args.mpd_address
            ))?,
        )),
        steam_id: args.steam_id,
    };

    let mut cs2_config_path = args.game_path.clone();
    cs2_config_path.push("game/csgo/cfg/gamestate_integration_cs2-mpd.cfg");
    tracing::info!("Current config path: {cs2_config_path:?}");
    let cs2_config = std::fs::read_to_string(cs2_config_path)?;
    tracing::trace!("Current config:\n{cs2_config}");

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(index)
    })
    .bind(args.listen_address)?
    .run()
    .await?;

    Ok(())
}
