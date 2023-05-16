pub use ints::*;
pub use strings::*;

pub mod strings {
    use serde::{de::Visitor, Deserialize, Serialize};
    use std::{fmt::Display, ops::Deref};

    #[derive(Debug, Clone, Serialize, PartialEq, Eq)]
    pub struct NonEmptyString(String);

    impl NonEmptyString {
        pub fn new(string: &str) -> Result<Self, NonEmptyStringError> {
            if string.is_empty() {
                return Err(NonEmptyStringError::EmptyString);
            }

            Ok(Self(string.into()))
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
            Self::new(&value)
        }
    }

    impl TryFrom<&str> for NonEmptyString {
        type Error = NonEmptyStringError;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Self::new(value)
        }
    }

    impl Display for NonEmptyString {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self)
        }
    }

    impl PartialEq<&str> for NonEmptyString {
        fn eq(&self, other: &&str) -> bool {
            self.0 == *other
        }
    }

    impl PartialEq<String> for NonEmptyString {
        fn eq(&self, other: &String) -> bool {
            self.0 == *other
        }
    }

    struct NonEmptyStringVisitor;

    impl<'de> Visitor<'de> for NonEmptyStringVisitor {
        type Value = NonEmptyString;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A non-empty string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Self::Value::new(v).map_err(|e| E::custom(e))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Self::Value::new(&v).map_err(|e| E::custom(e))
        }
    }

    impl<'de> Deserialize<'de> for NonEmptyString {
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
}

pub mod ints {
    use serde::{de::Visitor, Deserialize, Serialize};
    use std::{error::Error, ops::Deref};

    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
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

    impl<const MIN: i64> PartialEq<isize> for LowerBoundInt<MIN> {
        fn eq(&self, other: &isize) -> bool {
            self.0 as isize == *other
        }
    }

    impl<const MIN: i64> PartialEq<usize> for LowerBoundInt<MIN> {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }

    impl<const MIN: i64> PartialEq<i64> for LowerBoundInt<MIN> {
        fn eq(&self, other: &i64) -> bool {
            self.0 == *other
        }
    }

    impl<const MIN: i64> PartialEq<i32> for LowerBoundInt<MIN> {
        fn eq(&self, other: &i32) -> bool {
            self.0 as i32 == *other
        }
    }

    impl<const MIN: i64> PartialEq<i16> for LowerBoundInt<MIN> {
        fn eq(&self, other: &i16) -> bool {
            self.0 as i16 == *other
        }
    }

    impl<const MIN: i64> PartialEq<i8> for LowerBoundInt<MIN> {
        fn eq(&self, other: &i8) -> bool {
            self.0 as i8 == *other
        }
    }

    impl<const MIN: i64> PartialEq<u64> for LowerBoundInt<MIN> {
        fn eq(&self, other: &u64) -> bool {
            self.0 as u64 == *other
        }
    }

    impl<const MIN: i64> PartialEq<u32> for LowerBoundInt<MIN> {
        fn eq(&self, other: &u32) -> bool {
            self.0 as u32 == *other
        }
    }

    impl<const MIN: i64> PartialEq<u16> for LowerBoundInt<MIN> {
        fn eq(&self, other: &u16) -> bool {
            self.0 as u16 == *other
        }
    }

    impl<const MIN: i64> PartialEq<u8> for LowerBoundInt<MIN> {
        fn eq(&self, other: &u8) -> bool {
            self.0 as u8 == *other
        }
    }

    struct LowerBoundIntVisitor<const MIN: i64>;

    impl<'de, const MIN: i64> Visitor<'de> for LowerBoundIntVisitor<MIN> {
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

    impl<'de, const MIN: i64> Deserialize<'de> for LowerBoundInt<MIN> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_i64(LowerBoundIntVisitor)
        }
    }

    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
    pub struct UpperBoundInt<const MAX: i64>(i64);

    impl<const MAX: i64> UpperBoundInt<MAX> {
        pub fn new(val: i64) -> Result<Self, Box<dyn Error>> {
            if val > MAX {
                return Err(format!("Value must be less than or equal to {}", MAX).into());
            }

            Ok(Self(val))
        }
    }

    impl<const MAX: i64> Deref for UpperBoundInt<MAX> {
        type Target = i64;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<const MAX: i64> PartialEq<isize> for UpperBoundInt<MAX> {
        fn eq(&self, other: &isize) -> bool {
            self.0 as isize == *other
        }
    }

    impl<const MAX: i64> PartialEq<usize> for UpperBoundInt<MAX> {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }

    impl<const MAX: i64> PartialEq<i64> for UpperBoundInt<MAX> {
        fn eq(&self, other: &i64) -> bool {
            self.0 == *other
        }
    }

    impl<const MAX: i64> PartialEq<i32> for UpperBoundInt<MAX> {
        fn eq(&self, other: &i32) -> bool {
            self.0 as i32 == *other
        }
    }

    impl<const MAX: i64> PartialEq<i16> for UpperBoundInt<MAX> {
        fn eq(&self, other: &i16) -> bool {
            self.0 as i16 == *other
        }
    }

    impl<const MAX: i64> PartialEq<i8> for UpperBoundInt<MAX> {
        fn eq(&self, other: &i8) -> bool {
            self.0 as i8 == *other
        }
    }

    impl<const MAX: i64> PartialEq<u64> for UpperBoundInt<MAX> {
        fn eq(&self, other: &u64) -> bool {
            self.0 as u64 == *other
        }
    }

    impl<const MAX: i64> PartialEq<u32> for UpperBoundInt<MAX> {
        fn eq(&self, other: &u32) -> bool {
            self.0 as u32 == *other
        }
    }

    impl<const MAX: i64> PartialEq<u16> for UpperBoundInt<MAX> {
        fn eq(&self, other: &u16) -> bool {
            self.0 as u16 == *other
        }
    }

    impl<const MAX: i64> PartialEq<u8> for UpperBoundInt<MAX> {
        fn eq(&self, other: &u8) -> bool {
            self.0 as u8 == *other
        }
    }

    struct UpperBoundIntVisitor<const MAX: i64>;

    impl<'de, const MAX: i64> Visitor<'de> for UpperBoundIntVisitor<MAX> {
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

    impl<'de, const MAX: i64> Deserialize<'de> for UpperBoundInt<MAX> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_i64(UpperBoundIntVisitor)
        }
    }

    #[derive(Debug, Serialize, PartialEq, Eq, Clone, Copy)]
    pub struct BoundedInt<const MIN: i64, const MAX: i64>(i64);

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

    impl<const MIN: i64, const MAX: i64> PartialEq<isize> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &isize) -> bool {
            self.0 as isize == *other
        }
    }

    impl<const MIN: i64, const MAX: i64> PartialEq<usize> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &usize) -> bool {
            self.0 as usize == *other
        }
    }

    impl<const MIN: i64, const MAX: i64> PartialEq<i64> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &i64) -> bool {
            self.0 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<i32> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &i32) -> bool {
            self.0 as i32 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<i16> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &i16) -> bool {
            self.0 as i16 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<i8> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &i8) -> bool {
            self.0 as i8 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<u64> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &u64) -> bool {
            self.0 as u64 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<u32> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &u32) -> bool {
            self.0 as u32 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<u16> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &u16) -> bool {
            self.0 as u16 == *other
        }
    }
    impl<const MIN: i64, const MAX: i64> PartialEq<u8> for BoundedInt<MIN, MAX> {
        fn eq(&self, other: &u8) -> bool {
            self.0 as u8 == *other
        }
    }

    struct BoundedIntVisitor<const MIN: i64, const MAX: i64>;

    impl<'de, const MIN: i64, const MAX: i64> Visitor<'de> for BoundedIntVisitor<MIN, MAX> {
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

    impl<'de, const MIN: i64, const MAX: i64> Deserialize<'de> for BoundedInt<MIN, MAX> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            deserializer.deserialize_i64(BoundedIntVisitor)
        }
    }

    pub(crate) type NonNegative = LowerBoundInt<0>;
}

#[cfg(test)]
mod tests {
    use super::ints::*;
    use super::strings::*;

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
