pub fn set_mpd(mpd: &mut mpd::Client, state: mpd::State) -> Result<mpd::Status, mpd::error::Error> {
	match state {
		mpd::State::Play => {
			tracing::info!("Playing music");
			mpd.play()?;
			mpd.pause(false)?;
		}
		mpd::State::Pause => {
			tracing::info!("Pausing music");
			mpd.pause(true)?;
		}
		mpd::State::Stop => {
			tracing::info!("Stopping music");
			mpd.stop()?;
		}
	}

	let status = mpd.status()?;
	tracing::info!("New MPD status: {status:?}");

	Ok(status)
}
