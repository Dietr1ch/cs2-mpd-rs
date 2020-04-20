extern crate mpd;

use actix_web::{web, App, HttpResponse, HttpServer};
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

fn try_play_or_pause(game_data: &CsgoData) -> Result<(), mpd::error::Error> {
    let mut conn = mpd::Client::connect("127.0.0.1:6600")?;

    match game_data.round.phase.as_ref() {
        "freezetime" | "warmup" => {
            println!("Playing");
            conn.play()?;
            conn.pause(false)?;
        }
        _ => {
            if game_data.player.state.health <= 0 {
                println!("Playing");
                conn.play()?;
                conn.pause(false)?;
            } else {
                println!("Pausing");
                conn.pause(true)?;
            }
        }
    }

    println!("Status: {:?}", conn.status());
    Ok(())
}

fn play_or_pause(game_data: &CsgoData) {
    match game_data.player.steamid.as_ref() {
        "my_steam_id" => match try_play_or_pause(game_data) {
            Ok(_) => {}
            _ => {
                println!("Oh, no.");
            }
        },
        _ => {
            println!("Who's {:?}?", game_data.player.steamid);
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
    HttpServer::new(|| {
        App::new() //
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
