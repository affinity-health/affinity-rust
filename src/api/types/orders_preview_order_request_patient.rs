pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequestPatient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PreviewOrderRequestPatientAddress>,
    #[serde(rename = "clinicalProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_profile: Option<PreviewOrderRequestPatientClinicalProfile>,
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
    pub external_identities: Option<Vec<PreviewOrderRequestPatientExternalIdentitiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<PreviewOrderRequestPatientAddressesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Vec<PreviewOrderRequestPatientEncountersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<PreviewOrderRequestPatientGender>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<PreviewOrderRequestPatientMeasurementsItem>>,
    #[serde(default)]
    pub name: PreviewOrderRequestPatientName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<PreviewOrderRequestPatientProgramsItem>>,
}

impl PreviewOrderRequestPatient {
    pub fn builder() -> PreviewOrderRequestPatientBuilder {
        <PreviewOrderRequestPatientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPatientBuilder {
    address: Option<PreviewOrderRequestPatientAddress>,
    clinical_profile: Option<PreviewOrderRequestPatientClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<PreviewOrderRequestPatientExternalIdentitiesItem>>,
    addresses: Option<Vec<PreviewOrderRequestPatientAddressesItem>>,
    encounters: Option<Vec<PreviewOrderRequestPatientEncountersItem>>,
    gender: Option<PreviewOrderRequestPatientGender>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<PreviewOrderRequestPatientMeasurementsItem>>,
    name: Option<PreviewOrderRequestPatientName>,
    phone: Option<String>,
    programs: Option<Vec<PreviewOrderRequestPatientProgramsItem>>,
}

impl PreviewOrderRequestPatientBuilder {
    pub fn address(mut self, value: PreviewOrderRequestPatientAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn clinical_profile(mut self, value: PreviewOrderRequestPatientClinicalProfile) -> Self {
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
        value: Vec<PreviewOrderRequestPatientExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<PreviewOrderRequestPatientAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<PreviewOrderRequestPatientEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: PreviewOrderRequestPatientGender) -> Self {
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

    pub fn measurements(mut self, value: Vec<PreviewOrderRequestPatientMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: PreviewOrderRequestPatientName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(mut self, value: Vec<PreviewOrderRequestPatientProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPatient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_of_birth`](PreviewOrderRequestPatientBuilder::date_of_birth)
    /// - [`name`](PreviewOrderRequestPatientBuilder::name)
    pub fn build(self) -> Result<PreviewOrderRequestPatient, BuildError> {
        Ok(PreviewOrderRequestPatient {
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
