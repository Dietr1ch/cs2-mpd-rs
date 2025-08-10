use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PlayerState {
	pub health: u8,
	pub armor: u8,
	pub helmet: bool,

	pub flashed: u8,
	pub smoked: u8,
	pub burning: u8,

	pub round_kills: u8,
	#[serde(rename = "round_killhs")]
	pub round_hs_kills: u8,

	pub equip_value: u16,
	pub money: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameActivity {
	Menu,
	Playing,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Team {
	T,
	CT,
}

#[derive(Debug, Deserialize)]
pub struct PlayerData {
	#[serde(rename = "steamid")]
	pub steam_id: String,
	pub name: String,
	pub activity: GameActivity,

	#[serde(rename = "xpoverload")]
	pub xp_overload: Option<u8>,
	pub observer_slot: Option<u8>,

	pub team: Option<Team>,
	pub state: Option<PlayerState>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RoundPhase {
	Live,
	FreezeTime,
	WarmUp,
}

#[derive(Debug, Deserialize)]
pub struct RoundData {
	pub phase: RoundPhase,
}
#[derive(Debug, Deserialize)]
pub struct GameData {
	pub player: Option<PlayerData>,
	pub round: Option<RoundData>,
}
