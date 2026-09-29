pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateOrderRequestPatient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateOrderRequestPatientAddress>,
    #[serde(rename = "clinicalProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_profile: Option<CreateOrderRequestPatientClinicalProfile>,
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
    pub external_identities: Option<Vec<CreateOrderRequestPatientExternalIdentitiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<CreateOrderRequestPatientAddressesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Vec<CreateOrderRequestPatientEncountersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<CreateOrderRequestPatientGender>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<CreateOrderRequestPatientMeasurementsItem>>,
    #[serde(default)]
    pub name: CreateOrderRequestPatientName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<CreateOrderRequestPatientProgramsItem>>,
}

impl CreateOrderRequestPatient {
    pub fn builder() -> CreateOrderRequestPatientBuilder {
        <CreateOrderRequestPatientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPatientBuilder {
    address: Option<CreateOrderRequestPatientAddress>,
    clinical_profile: Option<CreateOrderRequestPatientClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<CreateOrderRequestPatientExternalIdentitiesItem>>,
    addresses: Option<Vec<CreateOrderRequestPatientAddressesItem>>,
    encounters: Option<Vec<CreateOrderRequestPatientEncountersItem>>,
    gender: Option<CreateOrderRequestPatientGender>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<CreateOrderRequestPatientMeasurementsItem>>,
    name: Option<CreateOrderRequestPatientName>,
    phone: Option<String>,
    programs: Option<Vec<CreateOrderRequestPatientProgramsItem>>,
}

impl CreateOrderRequestPatientBuilder {
    pub fn address(mut self, value: CreateOrderRequestPatientAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn clinical_profile(mut self, value: CreateOrderRequestPatientClinicalProfile) -> Self {
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
        value: Vec<CreateOrderRequestPatientExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<CreateOrderRequestPatientAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<CreateOrderRequestPatientEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: CreateOrderRequestPatientGender) -> Self {
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

    pub fn measurements(mut self, value: Vec<CreateOrderRequestPatientMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: CreateOrderRequestPatientName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<CreateOrderRequestPatientProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPatient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_of_birth`](CreateOrderRequestPatientBuilder::date_of_birth)
    /// - [`name`](CreateOrderRequestPatientBuilder::name)
    pub fn build(self) -> Result<CreateOrderRequestPatient, BuildError> {
        Ok(CreateOrderRequestPatient {
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
