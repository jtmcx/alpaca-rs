use ref_cast::RefCast;
use rug::Float;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(RefCast)]
#[repr(transparent)]
struct Wrapper(Float);

impl Serialize for Wrapper {
    /// todo: docs ...
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        super::float::serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for Wrapper {
    /// todo: docs ...
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Ok(Wrapper(super::float::deserialize(deserializer)?))
    }
}

/// todo: docs ...
pub fn serialize<S>(f: &Option<Float>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    match f {
        Some(f) => serializer.serialize_some(Wrapper::ref_cast(f)),
        None    => serializer.serialize_none(),
    }
}

/// todo: docs ...
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Float>, D::Error>
    where D: Deserializer<'de>
{
    Ok(Option::deserialize(deserializer)?.map(|Wrapper(f)| f))
}
