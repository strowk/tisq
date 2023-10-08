use kv::{Error, Key, Raw};
use uuid::Uuid;
pub(crate) struct Id(pub Uuid);

impl AsRef<[u8]> for Id {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Key<'_> for Id {
    fn from_raw_key(r: &'_ Raw) -> Result<Self, Error> {
        let uuid = Uuid::from_slice_le(r.as_ref())
            .map_err(|_| kv::Error::Message("Failed to parse uuid".to_owned()))?;
        Ok(Id(uuid))
    }

    fn to_raw_key(&self) -> Result<Raw, Error> {
        Ok(self.as_ref().into())
    }
}
