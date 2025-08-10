use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum GameActivity {
	Menu,
	Playing,
	#[serde(rename = "textinput")]
	Typing,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Team {
	T,
	CT,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoundPhase {
	Live,
	FreezeTime,
	WarmUp,
	Over,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct RoundData {
	pub phase: RoundPhase,
}
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GameData {
	pub player: Option<PlayerData>,
	pub round: Option<RoundData>,
}

#[cfg(test)]
mod tests {
	use googletest::prelude::*;
	use indoc::indoc;

	use super::*;

	mod game_data {
		use super::*;

		#[test_log::test(gtest)]
		fn parse_almost_empty() {
			expect_that!(
				serde_json::from_str::<GameData>("{}"),
				ok(matches_pattern!(GameData {
					player: none(),
					round: none(),
				}))
			);
		}

		#[test_log::test(gtest)]
		fn parse_nonempty() {
			expect_that!(
				serde_json::from_str::<GameData>(indoc! {r#"
					{
					  "player": {
					    "steamid": "1234",
					    "name": "n00b",
					    "activity": "textinput"
					  }
					}
				"#}),
				ok(matches_pattern!(GameData {
					player: some(pat!(PlayerData {
						steam_id: "1234",
						name: "n00b",
						activity: &GameActivity::Typing,
						..
					})),
					round: none(),
				}))
			);
		}
	}
}
