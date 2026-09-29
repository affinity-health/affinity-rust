pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum GetOrderResponseObject {
    #[serde(rename = "order")]
    Order,
}
impl fmt::Display for GetOrderResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Order => "order",
        };
        write!(f, "{}", s)
    }
}
