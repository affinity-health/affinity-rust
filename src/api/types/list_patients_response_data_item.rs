pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPatientsResponseDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ListPatientsResponseDataItemAddress>,
    #[serde(rename = "defaultShippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_address_id: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ListPatientsResponseDataItemShippingAddress>,
    #[serde(rename = "allergyReviewStatus")]
    pub allergy_review_status: ListPatientsResponseDataItemAllergyReviewStatus,
    #[serde(rename = "allergySummary")]
    #[serde(default)]
    pub allergy_summary: Vec<ListPatientsResponseDataItemAllergySummaryItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "clinicalProfile")]
    #[serde(default)]
    pub clinical_profile: ListPatientsResponseDataItemClinicalProfile,
    #[serde(rename = "dateOfBirth")]
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalIdentities")]
    #[serde(default)]
    pub external_identities: Vec<ListPatientsResponseDataItemExternalIdentitiesItem>,
    #[serde(default)]
    pub addresses: Vec<ListPatientsResponseDataItemAddressesItem>,
    #[serde(default)]
    pub encounters: Vec<ListPatientsResponseDataItemEncountersItem>,
    pub gender: ListPatientsResponseDataItemGender,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub location: ListPatientsResponseDataItemLocation,
    #[serde(rename = "locationId")]
    #[serde(default)]
    pub location_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(default)]
    pub measurements: Vec<ListPatientsResponseDataItemMeasurementsItem>,
    #[serde(default)]
    pub name: ListPatientsResponseDataItemName,
    pub object: ListPatientsResponseDataItemObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub programs: Vec<ListPatientsResponseDataItemProgramsItem>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    pub status: ListPatientsResponseDataItemStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl ListPatientsResponseDataItem {
    pub fn builder() -> ListPatientsResponseDataItemBuilder {
        <ListPatientsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPatientsResponseDataItemBuilder {
    address: Option<ListPatientsResponseDataItemAddress>,
    default_shipping_address_id: Option<String>,
    shipping_address: Option<ListPatientsResponseDataItemShippingAddress>,
    allergy_review_status: Option<ListPatientsResponseDataItemAllergyReviewStatus>,
    allergy_summary: Option<Vec<ListPatientsResponseDataItemAllergySummaryItem>>,
    created_at: Option<String>,
    clinical_profile: Option<ListPatientsResponseDataItemClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<ListPatientsResponseDataItemExternalIdentitiesItem>>,
    addresses: Option<Vec<ListPatientsResponseDataItemAddressesItem>>,
    encounters: Option<Vec<ListPatientsResponseDataItemEncountersItem>>,
    gender: Option<ListPatientsResponseDataItemGender>,
    id: Option<String>,
    livemode: Option<bool>,
    location: Option<ListPatientsResponseDataItemLocation>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<ListPatientsResponseDataItemMeasurementsItem>>,
    name: Option<ListPatientsResponseDataItemName>,
    object: Option<ListPatientsResponseDataItemObject>,
    phone: Option<String>,
    programs: Option<Vec<ListPatientsResponseDataItemProgramsItem>>,
    practice_id: Option<String>,
    status: Option<ListPatientsResponseDataItemStatus>,
    updated_at: Option<String>,
}

impl ListPatientsResponseDataItemBuilder {
    pub fn address(mut self, value: ListPatientsResponseDataItemAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn default_shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.default_shipping_address_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: ListPatientsResponseDataItemShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn allergy_review_status(
        mut self,
        value: ListPatientsResponseDataItemAllergyReviewStatus,
    ) -> Self {
        self.allergy_review_status = Some(value);
        self
    }

    pub fn allergy_summary(
        mut self,
        value: Vec<ListPatientsResponseDataItemAllergySummaryItem>,
    ) -> Self {
        self.allergy_summary = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn clinical_profile(mut self, value: ListPatientsResponseDataItemClinicalProfile) -> Self {
        self.clinical_profile = Some(value);
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn external_identities(
        mut self,
        value: Vec<ListPatientsResponseDataItemExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<ListPatientsResponseDataItemAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<ListPatientsResponseDataItemEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: ListPatientsResponseDataItemGender) -> Self {
        self.gender = Some(value);
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

    pub fn location(mut self, value: ListPatientsResponseDataItemLocation) -> Self {
        self.location = Some(value);
        self
    }

    pub fn location_id(mut self, value: impl Into<String>) -> Self {
        self.location_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn medical_record_number(mut self, value: impl Into<String>) -> Self {
        self.medical_record_number = Some(value.into());
        self
    }

    pub fn measurements(
        mut self,
        value: Vec<ListPatientsResponseDataItemMeasurementsItem>,
    ) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: ListPatientsResponseDataItemName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn object(mut self, value: ListPatientsResponseDataItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<ListPatientsResponseDataItemProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListPatientsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPatientsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergy_review_status`](ListPatientsResponseDataItemBuilder::allergy_review_status)
    /// - [`allergy_summary`](ListPatientsResponseDataItemBuilder::allergy_summary)
    /// - [`created_at`](ListPatientsResponseDataItemBuilder::created_at)
    /// - [`clinical_profile`](ListPatientsResponseDataItemBuilder::clinical_profile)
    /// - [`date_of_birth`](ListPatientsResponseDataItemBuilder::date_of_birth)
    /// - [`external_identities`](ListPatientsResponseDataItemBuilder::external_identities)
    /// - [`addresses`](ListPatientsResponseDataItemBuilder::addresses)
    /// - [`encounters`](ListPatientsResponseDataItemBuilder::encounters)
    /// - [`gender`](ListPatientsResponseDataItemBuilder::gender)
    /// - [`id`](ListPatientsResponseDataItemBuilder::id)
    /// - [`livemode`](ListPatientsResponseDataItemBuilder::livemode)
    /// - [`location`](ListPatientsResponseDataItemBuilder::location)
    /// - [`location_id`](ListPatientsResponseDataItemBuilder::location_id)
    /// - [`metadata`](ListPatientsResponseDataItemBuilder::metadata)
    /// - [`measurements`](ListPatientsResponseDataItemBuilder::measurements)
    /// - [`name`](ListPatientsResponseDataItemBuilder::name)
    /// - [`object`](ListPatientsResponseDataItemBuilder::object)
    /// - [`programs`](ListPatientsResponseDataItemBuilder::programs)
    /// - [`practice_id`](ListPatientsResponseDataItemBuilder::practice_id)
    /// - [`status`](ListPatientsResponseDataItemBuilder::status)
    /// - [`updated_at`](ListPatientsResponseDataItemBuilder::updated_at)
    pub fn build(self) -> Result<ListPatientsResponseDataItem, BuildError> {
        Ok(ListPatientsResponseDataItem {
            address: self.address,
            default_shipping_address_id: self.default_shipping_address_id,
            shipping_address: self.shipping_address,
            allergy_review_status: self
                .allergy_review_status
                .ok_or_else(|| BuildError::missing_field("allergy_review_status"))?,
            allergy_summary: self
                .allergy_summary
                .ok_or_else(|| BuildError::missing_field("allergy_summary"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            clinical_profile: self
                .clinical_profile
                .ok_or_else(|| BuildError::missing_field("clinical_profile"))?,
            date_of_birth: self
                .date_of_birth
                .ok_or_else(|| BuildError::missing_field("date_of_birth"))?,
            email: self.email,
            external_id: self.external_id,
            external_identities: self
                .external_identities
                .ok_or_else(|| BuildError::missing_field("external_identities"))?,
            addresses: self
                .addresses
                .ok_or_else(|| BuildError::missing_field("addresses"))?,
            encounters: self
                .encounters
                .ok_or_else(|| BuildError::missing_field("encounters"))?,
            gender: self
                .gender
                .ok_or_else(|| BuildError::missing_field("gender"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
            location_id: self
                .location_id
                .ok_or_else(|| BuildError::missing_field("location_id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            medical_record_number: self.medical_record_number,
            measurements: self
                .measurements
                .ok_or_else(|| BuildError::missing_field("measurements"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            phone: self.phone,
            programs: self
                .programs
                .ok_or_else(|| BuildError::missing_field("programs"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
