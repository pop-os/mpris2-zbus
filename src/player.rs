use crate::{
	bindings::{media_player::MediaPlayer2Proxy, player::PlayerProxy},
	error::{Error, Result},
	media_player::MediaPlayer,
};
use std::{
	fmt::{self, Display},
	ops::Deref,
	str::FromStr,
	time::Duration,
};
use zbus::{names::OwnedBusName, Connection};

#[derive(Debug, Clone)]
pub struct Player {
	proxy: PlayerProxy<'static>,
}

impl Player {
	/// Creates a new instance of the `org.mpris.MediaPlayer2.Player` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		PlayerProxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}

	/// Returns this player's `org.mpris.MediaPlayer2` instance
	pub async fn media_player(&self) -> Result<MediaPlayer> {
		let proxy = MediaPlayer2Proxy::builder(self.proxy.connection())
			.destination(self.proxy.destination().to_owned())?
			.build()
			.await?;
		Ok(proxy.into())
	}

	/// Seeks the specified duration forward.
	pub async fn seek_ahead(&self, duration: Duration) -> Result<bool> {
		if self.proxy.can_seek().await? {
			self.proxy.seek(duration.as_micros() as i64).await?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// Seeks the specified duration backwards.
	pub async fn seek_back(&self, duration: Duration) -> Result<bool> {
		if self.proxy.can_seek().await? {
			self.proxy.seek(-(duration.as_micros() as i64)).await?;
			Ok(true)
		} else {
			Ok(false)
		}
	}

	/// How far into the current track the player is.
	pub async fn position(&self) -> Result<Duration> {
		self.proxy
			.position()
			.await
			.map(|micros| Duration::from_micros(micros as u64))
			.map_err(Error::from)
	}

	/// Gets the current playback status of the player.
	pub async fn playback_status(&self) -> Result<PlaybackStatus> {
		self.proxy
			.playback_status()
			.await
			.map_err(Error::from)
			.and_then(|status| PlaybackStatus::from_str(&status))
	}

	/// Returns the range of playback rates available for the player.
	pub async fn available_rates(&self) -> Result<std::ops::RangeInclusive<f64>> {
		Ok(self.proxy.minimum_rate().await?..=self.proxy.maximum_rate().await?)
	}
}

impl Deref for Player {
	type Target = PlayerProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<PlayerProxy<'static>> for Player {
	fn from(proxy: PlayerProxy<'static>) -> Self {
		Self { proxy }
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PlaybackStatus {
	Playing,
	Paused,
	Stopped,
}

impl FromStr for PlaybackStatus {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self> {
		match s.to_lowercase().trim() {
			"playing" => Ok(Self::Playing),
			"paused" => Ok(Self::Paused),
			"stopped" => Ok(Self::Stopped),
			_ => Err(Error::InvalidEnum {
				got: s.to_string(),
				expected: &["Playing", "Paused", "Stopped"],
			}),
		}
	}
}

impl Display for PlaybackStatus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::Playing => "Playing",
				Self::Paused => "Paused",
				Self::Stopped => "Stopped",
			}
		)
	}
}
