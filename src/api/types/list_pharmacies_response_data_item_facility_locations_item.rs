pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPharmaciesResponseDataItemFacilityLocationsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl ListPharmaciesResponseDataItemFacilityLocationsItem {
    pub fn builder() -> ListPharmaciesResponseDataItemFacilityLocationsItemBuilder {
        <ListPharmaciesResponseDataItemFacilityLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPharmaciesResponseDataItemFacilityLocationsItemBuilder {
    city: Option<String>,
    country: Option<String>,
    name: Option<String>,
    state: Option<String>,
}

impl ListPharmaciesResponseDataItemFacilityLocationsItemBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPharmaciesResponseDataItemFacilityLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ListPharmaciesResponseDataItemFacilityLocationsItemBuilder::name)
    pub fn build(self) -> Result<ListPharmaciesResponseDataItemFacilityLocationsItem, BuildError> {
        Ok(ListPharmaciesResponseDataItemFacilityLocationsItem {
            city: self.city,
            country: self.country,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            state: self.state,
        })
    }
}
