pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePatientResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePatientResponseAddress>,
    #[serde(rename = "defaultShippingAddressId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_shipping_address_id: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreatePatientResponseShippingAddress>,
    #[serde(rename = "allergyReviewStatus")]
    pub allergy_review_status: CreatePatientResponseAllergyReviewStatus,
    #[serde(rename = "allergySummary")]
    #[serde(default)]
    pub allergy_summary: Vec<CreatePatientResponseAllergySummaryItem>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "clinicalProfile")]
    #[serde(default)]
    pub clinical_profile: CreatePatientResponseClinicalProfile,
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
    pub external_identities: Vec<CreatePatientResponseExternalIdentitiesItem>,
    #[serde(default)]
    pub addresses: Vec<CreatePatientResponseAddressesItem>,
    #[serde(default)]
    pub encounters: Vec<CreatePatientResponseEncountersItem>,
    pub gender: CreatePatientResponseGender,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub livemode: bool,
    pub location: CreatePatientResponseLocation,
    #[serde(rename = "locationId")]
    #[serde(default)]
    pub location_id: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(default)]
    pub measurements: Vec<CreatePatientResponseMeasurementsItem>,
    #[serde(default)]
    pub name: CreatePatientResponseName,
    pub object: CreatePatientResponseObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default)]
    pub programs: Vec<CreatePatientResponseProgramsItem>,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    pub status: CreatePatientResponseStatus,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl CreatePatientResponse {
    pub fn builder() -> CreatePatientResponseBuilder {
        <CreatePatientResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientResponseBuilder {
    address: Option<CreatePatientResponseAddress>,
    default_shipping_address_id: Option<String>,
    shipping_address: Option<CreatePatientResponseShippingAddress>,
    allergy_review_status: Option<CreatePatientResponseAllergyReviewStatus>,
    allergy_summary: Option<Vec<CreatePatientResponseAllergySummaryItem>>,
    created_at: Option<String>,
    clinical_profile: Option<CreatePatientResponseClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<CreatePatientResponseExternalIdentitiesItem>>,
    addresses: Option<Vec<CreatePatientResponseAddressesItem>>,
    encounters: Option<Vec<CreatePatientResponseEncountersItem>>,
    gender: Option<CreatePatientResponseGender>,
    id: Option<String>,
    livemode: Option<bool>,
    location: Option<CreatePatientResponseLocation>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<CreatePatientResponseMeasurementsItem>>,
    name: Option<CreatePatientResponseName>,
    object: Option<CreatePatientResponseObject>,
    phone: Option<String>,
    programs: Option<Vec<CreatePatientResponseProgramsItem>>,
    practice_id: Option<String>,
    status: Option<CreatePatientResponseStatus>,
    updated_at: Option<String>,
}

impl CreatePatientResponseBuilder {
    pub fn address(mut self, value: CreatePatientResponseAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn default_shipping_address_id(mut self, value: impl Into<String>) -> Self {
        self.default_shipping_address_id = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: CreatePatientResponseShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn allergy_review_status(
        mut self,
        value: CreatePatientResponseAllergyReviewStatus,
    ) -> Self {
        self.allergy_review_status = Some(value);
        self
    }

    pub fn allergy_summary(mut self, value: Vec<CreatePatientResponseAllergySummaryItem>) -> Self {
        self.allergy_summary = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn clinical_profile(mut self, value: CreatePatientResponseClinicalProfile) -> Self {
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
        value: Vec<CreatePatientResponseExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<CreatePatientResponseAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<CreatePatientResponseEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: CreatePatientResponseGender) -> Self {
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

    pub fn location(mut self, value: CreatePatientResponseLocation) -> Self {
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

    pub fn measurements(mut self, value: Vec<CreatePatientResponseMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: CreatePatientResponseName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn object(mut self, value: CreatePatientResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<CreatePatientResponseProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreatePatientResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`allergy_review_status`](CreatePatientResponseBuilder::allergy_review_status)
    /// - [`allergy_summary`](CreatePatientResponseBuilder::allergy_summary)
    /// - [`created_at`](CreatePatientResponseBuilder::created_at)
    /// - [`clinical_profile`](CreatePatientResponseBuilder::clinical_profile)
    /// - [`date_of_birth`](CreatePatientResponseBuilder::date_of_birth)
    /// - [`external_identities`](CreatePatientResponseBuilder::external_identities)
    /// - [`addresses`](CreatePatientResponseBuilder::addresses)
    /// - [`encounters`](CreatePatientResponseBuilder::encounters)
    /// - [`gender`](CreatePatientResponseBuilder::gender)
    /// - [`id`](CreatePatientResponseBuilder::id)
    /// - [`livemode`](CreatePatientResponseBuilder::livemode)
    /// - [`location`](CreatePatientResponseBuilder::location)
    /// - [`location_id`](CreatePatientResponseBuilder::location_id)
    /// - [`metadata`](CreatePatientResponseBuilder::metadata)
    /// - [`measurements`](CreatePatientResponseBuilder::measurements)
    /// - [`name`](CreatePatientResponseBuilder::name)
    /// - [`object`](CreatePatientResponseBuilder::object)
    /// - [`programs`](CreatePatientResponseBuilder::programs)
    /// - [`practice_id`](CreatePatientResponseBuilder::practice_id)
    /// - [`status`](CreatePatientResponseBuilder::status)
    /// - [`updated_at`](CreatePatientResponseBuilder::updated_at)
    pub fn build(self) -> Result<CreatePatientResponse, BuildError> {
        Ok(CreatePatientResponse {
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
