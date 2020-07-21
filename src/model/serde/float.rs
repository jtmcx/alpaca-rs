use rug::Float;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// todo ...
pub fn serialize<S>(f: &Float, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
    f.to_string_radix(10, None).serialize(serializer)
}

/// todo ...
pub fn deserialize<'de, D>(deserializer: D) -> Result<Float, D::Error>
    where D: Deserializer<'de>
{
    use serde::de::Error;

    // We use 53 bits of precision here. This provides us with
    // approximately 16 decimal places of precision. This is in line with
    // some of the other arbitrary-precision libraries that alpaca uses. I
    // personally wish that alpaca went with a fixed width decimal instead.
    // See https://github.com/shopspring/decimal.
    let precision = 53;

    let s = String::deserialize(deserializer)?;
    let partial = Float::parse(s).map_err(Error::custom)?;
    Ok(Float::with_val(precision, partial))
}
