pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetails {
    #[serde(default)]
    pub attributes:
        HashMap<String, RetrievePrescribingOptionsResponseCatalogCatalogDetailsAttributesValue>,
    #[serde(default)]
    pub directions: Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetails {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder {
        <RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder {
    attributes: Option<
        HashMap<String, RetrievePrescribingOptionsResponseCatalogCatalogDetailsAttributesValue>,
    >,
    directions: Option<Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem>>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder {
    pub fn attributes(
        mut self,
        value: HashMap<
            String,
            RetrievePrescribingOptionsResponseCatalogCatalogDetailsAttributesValue,
        >,
    ) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn directions(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem>,
    ) -> Self {
        self.directions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogCatalogDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attributes`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder::attributes)
    /// - [`directions`](RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder::directions)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogCatalogDetails, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogCatalogDetails {
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}
