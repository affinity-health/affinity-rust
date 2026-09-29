pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderRequestPatientGender {
    F,
    M,
    O,
    U,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderRequestPatientGender {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::F => serializer.serialize_str("f"),
            Self::M => serializer.serialize_str("m"),
            Self::O => serializer.serialize_str("o"),
            Self::U => serializer.serialize_str("u"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreviewOrderRequestPatientGender {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "f" => Ok(Self::F),
            "m" => Ok(Self::M),
            "o" => Ok(Self::O),
            "u" => Ok(Self::U),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreviewOrderRequestPatientGender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::F => write!(f, "f"),
            Self::M => write!(f, "m"),
            Self::O => write!(f, "o"),
            Self::U => write!(f, "u"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
