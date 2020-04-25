extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;
extern crate mpd;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

// CS:GO JSON
#[derive(Debug, Serialize, Deserialize)]
struct PlayerState {
    health: i32,
}
#[derive(Debug, Serialize, Deserialize)]
struct PlayerData {
    steamid: String,
    state: PlayerState,
}
#[derive(Debug, Serialize, Deserialize)]
struct RoundData {
    phase: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct CsgoData {
    player: PlayerData,
    round: RoundData,
}

enum MpdState {
    Play,
    Pause,
}
fn set_mpd(state: MpdState) -> Result<(), mpd::error::Error> {
    let env_mpd_address: &str = dotenv!("MPD_ADDRESS");
    let mut conn = mpd::Client::connect(env_mpd_address)?;

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

    HttpServer::new(|| {
        App::new() //
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
