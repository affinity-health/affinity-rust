pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdatePatientResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePatientResponseAddress>,
    #[serde(rename = "defaultShippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_address_id: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<UpdatePatientResponseShippingAddress>,
    #[serde(rename = "allergyReviewStatus")]
    pub allergy_review_status: UpdatePatientResponseAllergyReviewStatus,
    #[serde(rename = "allergySummary")]
    #[serde(default)]
    pub allergy_summary: Vec<UpdatePatientResponseAllergySummaryItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "clinicalProfile")]
    #[serde(default)]
    pub clinical_profile: UpdatePatientResponseClinicalProfile,
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
    pub external_identities: Vec<UpdatePatientResponseExternalIdentitiesItem>,
    #[serde(default)]
    pub addresses: Vec<UpdatePatientResponseAddressesItem>,
    #[serde(default)]
    pub encounters: Vec<UpdatePatientResponseEncountersItem>,
    pub gender: UpdatePatientResponseGender,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub location: UpdatePatientResponseLocation,
    #[serde(rename = "locationId")]
    #[serde(default)]
    pub location_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(default)]
    pub measurements: Vec<UpdatePatientResponseMeasurementsItem>,
    #[serde(default)]
    pub name: UpdatePatientResponseName,
    pub object: UpdatePatientResponseObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub programs: Vec<UpdatePatientResponseProgramsItem>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    pub status: UpdatePatientResponseStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl UpdatePatientResponse {
    pub fn builder() -> UpdatePatientResponseBuilder {
        <UpdatePatientResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientResponseBuilder {
    address: Option<UpdatePatientResponseAddress>,
    default_shipping_address_id: Option<String>,
    shipping_address: Option<UpdatePatientResponseShippingAddress>,
    allergy_review_status: Option<UpdatePatientResponseAllergyReviewStatus>,
    allergy_summary: Option<Vec<UpdatePatientResponseAllergySummaryItem>>,
    created_at: Option<String>,
    clinical_profile: Option<UpdatePatientResponseClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<UpdatePatientResponseExternalIdentitiesItem>>,
    addresses: Option<Vec<UpdatePatientResponseAddressesItem>>,
    encounters: Option<Vec<UpdatePatientResponseEncountersItem>>,
    gender: Option<UpdatePatientResponseGender>,
    id: Option<String>,
    livemode: Option<bool>,
    location: Option<UpdatePatientResponseLocation>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<UpdatePatientResponseMeasurementsItem>>,
    name: Option<UpdatePatientResponseName>,
    object: Option<UpdatePatientResponseObject>,
    phone: Option<String>,
    programs: Option<Vec<UpdatePatientResponseProgramsItem>>,
    practice_id: Option<String>,
    status: Option<UpdatePatientResponseStatus>,
    updated_at: Option<String>,
}

impl UpdatePatientResponseBuilder {
    pub fn address(mut self, value: UpdatePatientResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn default_shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.default_shipping_address_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: UpdatePatientResponseShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn allergy_review_status(
        mut self,
        value: UpdatePatientResponseAllergyReviewStatus,
    ) -> Self {
        self.allergy_review_status = Some(value);
        self
    }

    pub fn allergy_summary(mut self, value: Vec<UpdatePatientResponseAllergySummaryItem>) -> Self {
        self.allergy_summary = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn clinical_profile(mut self, value: UpdatePatientResponseClinicalProfile) -> Self {
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
        value: Vec<UpdatePatientResponseExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<UpdatePatientResponseAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<UpdatePatientResponseEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: UpdatePatientResponseGender) -> Self {
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

    pub fn location(mut self, value: UpdatePatientResponseLocation) -> Self {
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

    pub fn measurements(mut self, value: Vec<UpdatePatientResponseMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: UpdatePatientResponseName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn object(mut self, value: UpdatePatientResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<UpdatePatientResponseProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdatePatientResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergy_review_status`](UpdatePatientResponseBuilder::allergy_review_status)
    /// - [`allergy_summary`](UpdatePatientResponseBuilder::allergy_summary)
    /// - [`created_at`](UpdatePatientResponseBuilder::created_at)
    /// - [`clinical_profile`](UpdatePatientResponseBuilder::clinical_profile)
    /// - [`date_of_birth`](UpdatePatientResponseBuilder::date_of_birth)
    /// - [`external_identities`](UpdatePatientResponseBuilder::external_identities)
    /// - [`addresses`](UpdatePatientResponseBuilder::addresses)
    /// - [`encounters`](UpdatePatientResponseBuilder::encounters)
    /// - [`gender`](UpdatePatientResponseBuilder::gender)
    /// - [`id`](UpdatePatientResponseBuilder::id)
    /// - [`livemode`](UpdatePatientResponseBuilder::livemode)
    /// - [`location`](UpdatePatientResponseBuilder::location)
    /// - [`location_id`](UpdatePatientResponseBuilder::location_id)
    /// - [`metadata`](UpdatePatientResponseBuilder::metadata)
    /// - [`measurements`](UpdatePatientResponseBuilder::measurements)
    /// - [`name`](UpdatePatientResponseBuilder::name)
    /// - [`object`](UpdatePatientResponseBuilder::object)
    /// - [`programs`](UpdatePatientResponseBuilder::programs)
    /// - [`practice_id`](UpdatePatientResponseBuilder::practice_id)
    /// - [`status`](UpdatePatientResponseBuilder::status)
    /// - [`updated_at`](UpdatePatientResponseBuilder::updated_at)
    pub fn build(self) -> Result<UpdatePatientResponse, BuildError> {
        Ok(UpdatePatientResponse {
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
