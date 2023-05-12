use std::error::Error;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NonEmptyString(String);

impl NonEmptyString {
    fn new(string: String) -> Result<Self, Box<dyn Error>> {
        if string.is_empty() {
            return Err("`NonEmptyString` cannot be initialized with an empty string".into());
        }

        Ok(Self(string))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
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

pub(crate) type Symbol = NonEmptyString;
pub(crate) type Name = NonEmptyString;
pub(crate) type Description = NonEmptyString;
pub(crate) type Headquarters = NonEmptyString;
pub(crate) type Id = NonEmptyString;

#[derive(Debug)]
pub(crate) struct LowerBoundInt<const MIN: i64>(i64);

impl<const MIN: i64> LowerBoundInt<MIN> {
    fn new(val: i64) -> Result<Self, Box<dyn Error>> {
        if val < MIN {
            return Err(format!("Value must be greater than or equal to {}", MIN).into());
        }

        Ok(Self(val))
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
pub(crate) struct UpperBoundInt<const MAX: i64>(i64);

impl<const MAX: i64> UpperBoundInt<MAX> {
    fn new(val: i64) -> Result<Self, Box<dyn Error>> {
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

#[derive(Debug)]
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
        v1: LowerBoundInt<10>,
        v2: UpperBoundInt<100>,
        v3: BoundedInt<3, 5>,
    }

    #[derive(serde::Deserialize, Debug)]
    struct Tst2 {
        v: NonEmptyString,
    }

    #[test]
    fn can_deserialize_bounded_integers() {
        let data = r#"{"v1": 10, "v2": 100, "v3": 3}"#;

        let tst = serde_json::from_str::<Tst>(data).unwrap();

        // dbg!(tst);
    }

    #[test]
    fn can_deserialize_non_negative_strings() {
        let data = r#"{"v": "1"}"#;

        let tst = serde_json::from_str::<Tst2>(data).unwrap();
    }
}
