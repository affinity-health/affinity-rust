pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListOrderEventsResponseDataItemObject {
    #[serde(rename = "event")]
    Event,
}
impl fmt::Display for ListOrderEventsResponseDataItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Event => "event",
        };
        write!(f, "{}", s)
    }
}
