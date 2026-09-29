pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPharmaciesResponseDataItem {
    pub access: ListPharmaciesResponseDataItemAccess,
    #[serde(rename = "catalogItemCount")]
    #[serde(default)]
    pub catalog_item_count: i64,
    #[serde(rename = "facilityType")]
    #[serde(default)]
    pub facility_type: String,
    #[serde(rename = "facilityLocations")]
    #[serde(default)]
    pub facility_locations: Vec<ListPharmaciesResponseDataItemFacilityLocationsItem>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    #[serde(rename = "logoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(default)]
    pub name: String,
    pub object: ListPharmaciesResponseDataItemObject,
    #[serde(rename = "prescriptionsLast30Days")]
    #[serde(default)]
    pub prescriptions_last30days: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<ListPharmaciesResponseDataItemProfile>,
    #[serde(rename = "restrictedStates")]
    #[serde(default)]
    pub restricted_states: Vec<String>,
    #[serde(rename = "shippingOptions")]
    #[serde(default)]
    pub shipping_options: Vec<ListPharmaciesResponseDataItemShippingOptionsItem>,
    #[serde(rename = "supportedStates")]
    #[serde(default)]
    pub supported_states: Vec<String>,
}

impl ListPharmaciesResponseDataItem {
    pub fn builder() -> ListPharmaciesResponseDataItemBuilder {
        <ListPharmaciesResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPharmaciesResponseDataItemBuilder {
    access: Option<ListPharmaciesResponseDataItemAccess>,
    catalog_item_count: Option<i64>,
    facility_type: Option<String>,
    facility_locations: Option<Vec<ListPharmaciesResponseDataItemFacilityLocationsItem>>,
    id: Option<String>,
    livemode: Option<bool>,
    logo_url: Option<String>,
    name: Option<String>,
    object: Option<ListPharmaciesResponseDataItemObject>,
    prescriptions_last30days: Option<i64>,
    profile: Option<ListPharmaciesResponseDataItemProfile>,
    restricted_states: Option<Vec<String>>,
    shipping_options: Option<Vec<ListPharmaciesResponseDataItemShippingOptionsItem>>,
    supported_states: Option<Vec<String>>,
}

impl ListPharmaciesResponseDataItemBuilder {
    pub fn access(mut self, value: ListPharmaciesResponseDataItemAccess) -> Self {
        self.access = Some(value);
        self
    }

    pub fn catalog_item_count(mut self, value: i64) -> Self {
        self.catalog_item_count = Some(value);
        self
    }

    pub fn facility_type(mut self, value: impl Into<String>) -> Self {
        self.facility_type = Some(value.into());
        self
    }

    pub fn facility_locations(
        mut self,
        value: Vec<ListPharmaciesResponseDataItemFacilityLocationsItem>,
    ) -> Self {
        self.facility_locations = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: ListPharmaciesResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn prescriptions_last30days(mut self, value: i64) -> Self {
        self.prescriptions_last30days = Some(value);
        self
    }

    pub fn profile(mut self, value: ListPharmaciesResponseDataItemProfile) -> Self {
        self.profile = Some(value);
        self
    }

    pub fn restricted_states(mut self, value: Vec<String>) -> Self {
        self.restricted_states = Some(value);
        self
    }

    pub fn shipping_options(
        mut self,
        value: Vec<ListPharmaciesResponseDataItemShippingOptionsItem>,
    ) -> Self {
        self.shipping_options = Some(value);
        self
    }

    pub fn supported_states(mut self, value: Vec<String>) -> Self {
        self.supported_states = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPharmaciesResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access`](ListPharmaciesResponseDataItemBuilder::access)
    /// - [`catalog_item_count`](ListPharmaciesResponseDataItemBuilder::catalog_item_count)
    /// - [`facility_type`](ListPharmaciesResponseDataItemBuilder::facility_type)
    /// - [`facility_locations`](ListPharmaciesResponseDataItemBuilder::facility_locations)
    /// - [`id`](ListPharmaciesResponseDataItemBuilder::id)
    /// - [`livemode`](ListPharmaciesResponseDataItemBuilder::livemode)
    /// - [`name`](ListPharmaciesResponseDataItemBuilder::name)
    /// - [`object`](ListPharmaciesResponseDataItemBuilder::object)
    /// - [`prescriptions_last30days`](ListPharmaciesResponseDataItemBuilder::prescriptions_last30days)
    /// - [`restricted_states`](ListPharmaciesResponseDataItemBuilder::restricted_states)
    /// - [`shipping_options`](ListPharmaciesResponseDataItemBuilder::shipping_options)
    /// - [`supported_states`](ListPharmaciesResponseDataItemBuilder::supported_states)
    pub fn build(self) -> Result<ListPharmaciesResponseDataItem, BuildError> {
        Ok(ListPharmaciesResponseDataItem {
            access: self
                .access
                .ok_or_else(|| BuildError::missing_field("access"))?,
            catalog_item_count: self
                .catalog_item_count
                .ok_or_else(|| BuildError::missing_field("catalog_item_count"))?,
            facility_type: self
                .facility_type
                .ok_or_else(|| BuildError::missing_field("facility_type"))?,
            facility_locations: self
                .facility_locations
                .ok_or_else(|| BuildError::missing_field("facility_locations"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            logo_url: self.logo_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            prescriptions_last30days: self
                .prescriptions_last30days
                .ok_or_else(|| BuildError::missing_field("prescriptions_last30days"))?,
            profile: self.profile,
            restricted_states: self
                .restricted_states
                .ok_or_else(|| BuildError::missing_field("restricted_states"))?,
            shipping_options: self
                .shipping_options
                .ok_or_else(|| BuildError::missing_field("shipping_options"))?,
            supported_states: self
                .supported_states
                .ok_or_else(|| BuildError::missing_field("supported_states"))?,
        })
    }
}
