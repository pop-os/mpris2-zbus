use crate::{
	bindings::{media_player::MediaPlayer2Proxy, player::PlayerProxy},
	error::{Error, Result},
	player::Player,
};
use std::ops::Deref;
use zbus::{fdo::DBusProxy, names::OwnedBusName, Connection};

pub struct MediaPlayer {
	proxy: MediaPlayer2Proxy<'static>,
}

impl MediaPlayer {
	/// Creates a new instance of the `org.mpris.MediaPlayer2` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		MediaPlayer2Proxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}

	/// Gets the names of all the MPRIS players that are available on the current session.
	pub async fn available_players(connection: &Connection) -> Result<Vec<OwnedBusName>> {
		let dbus = DBusProxy::builder(connection)
			.path("/org/freedesktop/DBus")?
			.build()
			.await?;
		let mut players = Vec::new();
		for name in dbus.list_names().await? {
			if name.starts_with("org.mpris.MediaPlayer2.") {
				players.push(name);
			}
		}
		Ok(players)
	}

	/// Gets a new instance of all the MPRIS players that are available on the current session.
	pub async fn new_all(connection: &Connection) -> Result<Vec<Self>> {
		let players = Self::available_players(connection).await?;
		let mut instances = Vec::with_capacity(players.len());
		for player in players {
			instances.push(Self::new(connection, player).await?);
		}
		Ok(instances)
	}

	/// Returns an instance to the `org.mpris.MediaPlayer2.Player` interface of this object.
	pub async fn player(&self) -> Result<Player> {
		let proxy = PlayerProxy::builder(self.proxy.connection())
			.destination(self.proxy.destination().to_owned())?
			.build()
			.await?;
		Ok(proxy.into())
	}
}

impl Deref for MediaPlayer {
	type Target = MediaPlayer2Proxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<MediaPlayer2Proxy<'static>> for MediaPlayer {
	fn from(proxy: MediaPlayer2Proxy<'static>) -> Self {
		Self { proxy }
	}
}
