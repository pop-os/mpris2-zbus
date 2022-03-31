use miette::{IntoDiagnostic, Result, WrapErr};
use mpris2_zbus::media_player::MediaPlayer;
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
	let connection = Connection::session()
		.await
		.into_diagnostic()
		.wrap_err("Failed to establish session D-Bus connection")?;
	let media_players = MediaPlayer::new_all(&connection)
		.await
		.into_diagnostic()
		.wrap_err("Failed get available players")?;
	for media_player in media_players {
		let name = media_player
			.identity()
			.await
			.into_diagnostic()
			.wrap_err("Failed to get identity for media player")?;
		let desktop_entry = media_player
			.desktop_entry()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get desktop entry for media player '{}'", name))?;
		println!("{} ({})", name, desktop_entry);
		let player = media_player
			.player()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get player for media player '{}'", name))?;
		let playback_status = player
			.playback_status()
			.await
			.into_diagnostic()
			.wrap_err_with(|| {
				format!("Failed to get playback status for media player '{}'", name)
			})?;
		println!("\tPlayback Status: {}", playback_status);
		let position = player
			.position()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get position for media player '{}'", name))?
			.map(|s| format!("{} seconds", s.as_secs_f32()))
			.unwrap_or_else(|| "N/A".to_owned());
		println!("\tPosition: {}", position);
	}
	Ok(())
}
