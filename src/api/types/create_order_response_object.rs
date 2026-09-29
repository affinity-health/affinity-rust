pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOrderResponseObject {
    #[serde(rename = "order")]
    Order,
}
impl fmt::Display for CreateOrderResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Order => "order",
        };
        write!(f, "{}", s)
    }
}
