mod csgo_data;

extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate mpd;

use actix_web::{web, App, HttpResponse, HttpServer};
use csgo_data::CsgoData;
use dotenv::dotenv;

enum MpdState {
    Play,
    Pause,
}
fn set_mpd(state: MpdState) -> Result<(), mpd::error::Error> {
    let mut conn = mpd::Client::connect(dotenv!("MPD_ADDRESS"))?;

    match state {
        MpdState::Play => {
            println!("Playing");
            conn.play()?;
            conn.pause(false)?;
        }
        MpdState::Pause => {
            println!("Pausing");
            conn.pause(true)?;
        }
    }

    println!("Status: {:?}", conn.status());
    Ok(())
}

fn try_play_or_pause(game_data: &CsgoData) -> Result<(), mpd::error::Error> {
    match game_data.round.phase.as_ref() {
        "freezetime" | "warmup" => set_mpd(MpdState::Play),
        _ => {
            if game_data.player.state.health <= 0 {
                set_mpd(MpdState::Play)
            } else {
                set_mpd(MpdState::Pause)
            }
        }
    }
}

fn play_or_pause(game_data: &CsgoData) {
    let env_steam_id: &str = dotenv!("STEAM_ID");

    if game_data.player.steamid.as_str() != env_steam_id {
        println!("Who's {:?}?", game_data.player.steamid);
        return;
    }

    match try_play_or_pause(game_data) {
        Ok(_) => {}
        _ => {
            println!("Oh, no.");
        }
    }
}

async fn index(game_data: web::Json<CsgoData>) -> HttpResponse {
    println!("model: {:?}", &game_data);
    play_or_pause(&game_data);

    HttpResponse::Ok().json("")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().unwrap();

    println!("MPD_ADDRESS: {}", dotenv!("MPD_ADDRESS"));
    println!("STEAM_ID: {}", dotenv!("STEAM_ID"));

    HttpServer::new(|| {
        App::new() //
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
