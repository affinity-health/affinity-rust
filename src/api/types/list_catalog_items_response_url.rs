pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListCatalogItemsResponseUrl {
    #[serde(rename = "/v1/catalog/items")]
    V1CatalogItems,
}
impl fmt::Display for ListCatalogItemsResponseUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1CatalogItems => "/v1/catalog/items",
        };
        write!(f, "{}", s)
    }
}
