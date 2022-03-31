use crate::{
	bindings::{media_player::MediaPlayer2Proxy, player::PlayerProxy},
	media_player::MediaPlayer,
};
use std::ops::Deref;
use zbus::{names::OwnedBusName, Connection, Result};

pub struct Player {
	proxy: PlayerProxy<'static>,
}

impl Player {
	/// Returns this player's `org.mpris.MediaPlayer2` instance
	pub async fn media_player(&self) -> Result<MediaPlayer> {
		let proxy = MediaPlayer2Proxy::builder(self.proxy.connection())
			.destination(self.proxy.destination().to_owned())?
			.build()
			.await?;
		Ok(proxy.into())
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
