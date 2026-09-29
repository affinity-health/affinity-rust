pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrievePrescribingOptionsResponseCatalogObject {
    #[serde(rename = "catalog_item")]
    CatalogItem,
}
impl fmt::Display for RetrievePrescribingOptionsResponseCatalogObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CatalogItem => "catalog_item",
        };
        write!(f, "{}", s)
    }
}
