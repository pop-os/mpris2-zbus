pub mod bindings;
pub mod error;
pub mod media_player;
pub mod player;
pub mod metadata;

pub(crate) fn handle_optional<T>(input: zbus::Result<T>) -> error::Result<Option<T>> {
	match input {
		Ok(input) => Ok(Some(input)),
		Err(zbus::Error::FDO(fdo_error))
			if matches!(*fdo_error, zbus::fdo::Error::NotSupported(_)) =>
		{
			Ok(None)
		}
		Err(err) => Err(error::Error::from(err)),
	}
}
