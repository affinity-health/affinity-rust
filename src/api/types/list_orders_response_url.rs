pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListOrdersResponseUrl {
    #[serde(rename = "/v1/orders")]
    V1Orders,
}
impl fmt::Display for ListOrdersResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1Orders => "/v1/orders",
        };
        write!(f, "{}", s)
    }
}
