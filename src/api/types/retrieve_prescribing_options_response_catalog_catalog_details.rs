pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogCatalogDetails {
    /// Confirmed physical containers and contents. Empty or absent means container count cannot be inferred from dispense quantity.
    #[serde(rename = "packageComponents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_components:
        Option<Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem>>,
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
    package_components:
        Option<Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem>>,
    attributes: Option<
        HashMap<String, RetrievePrescribingOptionsResponseCatalogCatalogDetailsAttributesValue>,
    >,
    directions: Option<Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsDirectionsItem>>,
}

impl RetrievePrescribingOptionsResponseCatalogCatalogDetailsBuilder {
    pub fn package_components(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogCatalogDetailsPackageComponentsItem>,
    ) -> Self {
        self.package_components = Some(value);
        self
    }

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
            package_components: self.package_components,
            attributes: self
                .attributes
                .ok_or_else(|| BuildError::missing_field("attributes"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
        })
    }
}
