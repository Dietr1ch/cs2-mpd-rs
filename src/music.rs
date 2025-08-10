#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MpdState {
	Play,
	Pause,
}

pub fn set_mpd(mpd: &mut mpd::Client, state: MpdState) -> Result<(), mpd::error::Error> {
	match state {
		MpdState::Play => {
			tracing::info!("Playing music");
			mpd.play()?;
			mpd.pause(false)?;
		}
		MpdState::Pause => {
			tracing::info!("Pausing music");
			mpd.pause(true)?;
		}
	}

	tracing::info!("Status: {:?}", mpd.status());
	Ok(())
}
