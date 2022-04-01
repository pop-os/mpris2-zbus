// SPDX-License-Identifier: MPL-2.0
use crate::{
	bindings::track_list::TrackListProxy,
	error::{Error, Result},
	metadata::Metadata,
	track::Track,
};
use std::ops::Deref;
use zbus::{names::OwnedBusName, Connection};

#[derive(Debug, Clone)]
pub struct TrackList {
	proxy: TrackListProxy<'static>,
}

impl TrackList {
	/// Creates a new instance of the `org.mpris.MediaPlayer2.TrackList` interface.
	pub async fn new(connection: &Connection, name: OwnedBusName) -> Result<Self> {
		TrackListProxy::builder(connection)
			.destination(name)?
			.build()
			.await
			.map(Self::from)
			.map_err(Error::from)
	}

	/// Adds a new track to this track list.
	pub async fn add_track<S: ToString>(
		&self,
		uri: S,
		after: &Track,
		set_as_current: bool,
	) -> Result<()> {
		let uri = uri.to_string();
		self.proxy
			.add_track(&uri, after, set_as_current)
			.await
			.map_err(Error::from)
	}

	/// Gets the metadata of the given tracks.
	pub async fn get_tracks_metadata<T: AsRef<[Track]>>(&self, tracks: T) -> Result<Vec<Metadata>> {
		self.proxy
			.get_tracks_metadata(
				tracks
					.as_ref()
					.iter()
					.cloned()
					.map(Track::into_static_path)
					.collect(),
			)
			.await
			.map(|x| x.into_iter().map(Metadata::from).collect())
			.map_err(Error::from)
	}

	/// Goes to the specified track.
	pub async fn go_to(&self, track: &Track) -> Result<()> {
		self.proxy.go_to(track).await.map_err(Error::from)
	}

	/// Removes the specified track.
	pub async fn remove(&self, track: &Track) -> Result<()> {
		self.proxy.remove_track(track).await.map_err(Error::from)
	}

	/// Returns a list of all available [Track]s.
	pub async fn tracks(&self) -> Result<Vec<Track>> {
		self.proxy
			.tracks()
			.await
			.map(|x| x.into_iter().map(Track::from).collect())
			.map_err(Error::from)
	}
}

impl Deref for TrackList {
	type Target = TrackListProxy<'static>;

	fn deref(&self) -> &Self::Target {
		&self.proxy
	}
}

impl From<TrackListProxy<'static>> for TrackList {
	fn from(proxy: TrackListProxy<'static>) -> Self {
		Self { proxy }
	}
}
