use std::{error::Error, ops::Deref};

#[derive(Debug, Clone, serde::Serialize)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    pub fn new(string: String) -> Result<Self, NonEmptyStringError> {
        if string.is_empty() {
            return Err(NonEmptyStringError::EmptyString);
        }

        Ok(Self(string))
    }
}

impl Deref for NonEmptyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(thiserror::Error, Debug)]
pub enum NonEmptyStringError {
    #[error("`NonEmptyString` cannot be initialized with an empty string")]
    EmptyString,
}

impl TryFrom<String> for NonEmptyString {
    type Error = NonEmptyStringError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for NonEmptyString {
    type Error = NonEmptyStringError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_owned())
    }
}

struct NonEmptyStringVisitor;

impl<'de> serde::de::Visitor<'de> for NonEmptyStringVisitor {
    type Value = NonEmptyString;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("A non-empty string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v.into()).map_err(|e| E::custom(e))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v).map_err(|e| E::custom(e))
    }
}

impl<'de> serde::Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(NonEmptyStringVisitor)
    }
}

pub type Symbol = NonEmptyString;
pub type Name = NonEmptyString;
pub type Description = NonEmptyString;
pub type Headquarters = NonEmptyString;
pub type Id = NonEmptyString;

#[derive(Debug, serde::Serialize)]
pub struct LowerBoundInt<const MIN: i64>(i64);

impl<const MIN: i64> LowerBoundInt<MIN> {
    pub fn new(val: i64) -> Result<Self, Box<dyn Error>> {
        if val < MIN {
            return Err(format!("Value must be greater than or equal to {}", MIN).into());
        }

        Ok(Self(val))
    }
}

impl<const MIN: i64> Deref for LowerBoundInt<MIN> {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct LowerBoundIntVisitor<const MIN: i64>;

impl<'de, const MIN: i64> serde::de::Visitor<'de> for LowerBoundIntVisitor<MIN> {
    type Value = LowerBoundInt<MIN>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an integer greater than or equal to {}", MIN))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v).map_err(|e| E::custom(e))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v as i64).map_err(|e| E::custom(e))
    }
}

impl<'de, const MIN: i64> serde::Deserialize<'de> for LowerBoundInt<MIN> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(LowerBoundIntVisitor)
    }
}

#[derive(Debug)]
pub struct UpperBoundInt<const MAX: i64>(i64);

impl<const MAX: i64> UpperBoundInt<MAX> {
    pub fn new(val: i64) -> Result<Self, Box<dyn Error>> {
        if val > MAX {
            return Err(format!("Value must be less than or equal to {}", MAX).into());
        }

        Ok(Self(val))
    }
}

struct UpperBoundIntVisitor<const MAX: i64>;

impl<'de, const MAX: i64> serde::de::Visitor<'de> for UpperBoundIntVisitor<MAX> {
    type Value = UpperBoundInt<MAX>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an integer less than or equal to {}", MAX))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v).map_err(|e| E::custom(e))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v > i64::MAX as u64 {
            return Err(E::custom("The value can't be converted into i64 safely"));
        }

        Self::Value::new(v as i64).map_err(|e| E::custom(e))
    }
}

impl<'de, const MAX: i64> serde::Deserialize<'de> for UpperBoundInt<MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(UpperBoundIntVisitor)
    }
}

#[derive(Debug, serde::Serialize)]
pub(crate) struct BoundedInt<const MIN: i64, const MAX: i64>(i64);

impl<const MIN: i64, const MAX: i64> BoundedInt<MIN, MAX> {
    fn new(val: i64) -> Result<Self, Box<dyn Error>> {
        if val > MAX || val < MIN {
            return Err(format!("Value must be in the inclusive range [{MIN},{MAX}]").into());
            // return Err(format!("Value must be in between (or equal to) {MIN} and {MAX}").into());
        }

        Ok(Self(val))
    }
}

impl<const MIN: i64, const MAX: i64> Deref for BoundedInt<MIN, MAX> {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct BoundedIntVisitor<const MIN: i64, const MAX: i64>;

impl<'de, const MIN: i64, const MAX: i64> serde::de::Visitor<'de> for BoundedIntVisitor<MIN, MAX> {
    type Value = BoundedInt<MAX, MIN>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(&format!("an integer within the range [{MIN}, {MAX}]"))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::new(v).map_err(|e| E::custom(e))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v > i64::MAX as u64 {
            return Err(E::custom("The value can't be converted into i64 safely"));
        }

        Self::Value::new(v as i64).map_err(|e| E::custom(e))
    }
}

impl<'de, const MIN: i64, const MAX: i64> serde::Deserialize<'de> for BoundedInt<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(BoundedIntVisitor)
    }
}

pub(crate) type NonNegative = LowerBoundInt<0>;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Deserialize, Debug)]
    struct Tst {
        _v1: LowerBoundInt<10>,
        _v2: UpperBoundInt<100>,
        _v3: BoundedInt<3, 5>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct Tst2 {
        _v: NonEmptyString,
    }

    #[test]
    fn can_deserialize_bounded_integers() {
        let data = r#"{"_v1": 10, "_v2": 100, "_v3": 3}"#;

        serde_json::from_str::<Tst>(data).unwrap();
    }

    #[test]
    fn can_deserialize_non_empty_strings() {
        let data = r#"{"_v": "1"}"#;

        serde_json::from_str::<Tst2>(data).unwrap();
    }
}
