pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePatientRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePatientRequestAddress>,
    #[serde(rename = "clinicalProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_profile: Option<CreatePatientRequestClinicalProfile>,
    #[serde(rename = "dateOfBirth")]
    #[serde(default)]
    pub date_of_birth: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identities: Option<Vec<CreatePatientRequestExternalIdentitiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<CreatePatientRequestAddressesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Vec<CreatePatientRequestEncountersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<CreatePatientRequestGender>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<CreatePatientRequestMeasurementsItem>>,
    #[serde(default)]
    pub name: CreatePatientRequestName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<CreatePatientRequestProgramsItem>>,
}

impl CreatePatientRequest {
    pub fn builder() -> CreatePatientRequestBuilder {
        <CreatePatientRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePatientRequestBuilder {
    address: Option<CreatePatientRequestAddress>,
    clinical_profile: Option<CreatePatientRequestClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<CreatePatientRequestExternalIdentitiesItem>>,
    addresses: Option<Vec<CreatePatientRequestAddressesItem>>,
    encounters: Option<Vec<CreatePatientRequestEncountersItem>>,
    gender: Option<CreatePatientRequestGender>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<CreatePatientRequestMeasurementsItem>>,
    name: Option<CreatePatientRequestName>,
    phone: Option<String>,
    programs: Option<Vec<CreatePatientRequestProgramsItem>>,
}

impl CreatePatientRequestBuilder {
    pub fn address(mut self, value: CreatePatientRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn clinical_profile(mut self, value: CreatePatientRequestClinicalProfile) -> Self {
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
        value: Vec<CreatePatientRequestExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<CreatePatientRequestAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<CreatePatientRequestEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: CreatePatientRequestGender) -> Self {
        self.gender = Some(value);
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

    pub fn measurements(mut self, value: Vec<CreatePatientRequestMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: CreatePatientRequestName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<CreatePatientRequestProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePatientRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_of_birth`](CreatePatientRequestBuilder::date_of_birth)
    /// - [`name`](CreatePatientRequestBuilder::name)
    pub fn build(self) -> Result<CreatePatientRequest, BuildError> {
        Ok(CreatePatientRequest {
            address: self.address,
            clinical_profile: self.clinical_profile,
            date_of_birth: self
                .date_of_birth
                .ok_or_else(|| BuildError::missing_field("date_of_birth"))?,
            email: self.email,
            external_id: self.external_id,
            external_identities: self.external_identities,
            addresses: self.addresses,
            encounters: self.encounters,
            gender: self.gender,
            location_id: self.location_id,
            metadata: self.metadata,
            medical_record_number: self.medical_record_number,
            measurements: self.measurements,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
            programs: self.programs,
        })
    }
}
