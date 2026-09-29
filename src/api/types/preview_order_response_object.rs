pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PreviewOrderResponseObject {
    #[serde(rename = "order_preview")]
    OrderPreview,
}
impl fmt::Display for PreviewOrderResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OrderPreview => "order_preview",
        };
        write!(f, "{}", s)
    }
}
