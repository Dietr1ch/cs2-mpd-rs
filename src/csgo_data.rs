use serde::{Deserialize, Serialize};

// CS:GO JSON
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub health: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerData {
    pub steamid: String,
    pub state: PlayerState,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RoundData {
    pub phase: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CsgoData {
    pub player: PlayerData,
    pub round: RoundData,
}
