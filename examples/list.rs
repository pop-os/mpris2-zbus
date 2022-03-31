// SPDX-License-Identifier: MPL-2.0
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
			.map(|s| format!("{} seconds", s.as_seconds_f32()))
			.unwrap_or_else(|| "N/A".to_owned());
		println!("\tPosition: {}", position);
		if !player
			.can_seek()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get can_seek for media player '{}'", name))?
		{
			println!("\tDoesn't support seeking");
		} else {
			println!("\tSupports seeking");
		}
		let supported_rates = player
			.available_rates()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get supported rates for media player '{}'", name))?
			.map(|s| format!("{}x through {}x", s.start(), s.end()))
			.unwrap_or_else(|| "N/A".to_owned());
		println!("\tSupported Rates: {}", supported_rates);
		let current_rate = player
			.rate()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get current rate for media player '{}'", name))?
			.map(|s| format!("{}x", s))
			.unwrap_or_else(|| "N/A".to_owned());
		println!("\tCurrent Rate: {}", current_rate);
		let metadata = player
			.metadata()
			.await
			.into_diagnostic()
			.wrap_err_with(|| format!("Failed to get metadata for media player '{}'", name))?;
		println!("\tMetadata:\n{}", metadata);
	}
	Ok(())
}
