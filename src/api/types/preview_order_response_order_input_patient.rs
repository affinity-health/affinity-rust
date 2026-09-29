pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponseOrderInputPatient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PreviewOrderResponseOrderInputPatientAddress>,
    #[serde(rename = "clinicalProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_profile: Option<PreviewOrderResponseOrderInputPatientClinicalProfile>,
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
    pub external_identities:
        Option<Vec<PreviewOrderResponseOrderInputPatientExternalIdentitiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<PreviewOrderResponseOrderInputPatientAddressesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Vec<PreviewOrderResponseOrderInputPatientEncountersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<PreviewOrderResponseOrderInputPatientGender>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<PreviewOrderResponseOrderInputPatientMeasurementsItem>>,
    #[serde(default)]
    pub name: PreviewOrderResponseOrderInputPatientName,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<PreviewOrderResponseOrderInputPatientProgramsItem>>,
}

impl PreviewOrderResponseOrderInputPatient {
    pub fn builder() -> PreviewOrderResponseOrderInputPatientBuilder {
        <PreviewOrderResponseOrderInputPatientBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPatientBuilder {
    address: Option<PreviewOrderResponseOrderInputPatientAddress>,
    clinical_profile: Option<PreviewOrderResponseOrderInputPatientClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<PreviewOrderResponseOrderInputPatientExternalIdentitiesItem>>,
    addresses: Option<Vec<PreviewOrderResponseOrderInputPatientAddressesItem>>,
    encounters: Option<Vec<PreviewOrderResponseOrderInputPatientEncountersItem>>,
    gender: Option<PreviewOrderResponseOrderInputPatientGender>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<PreviewOrderResponseOrderInputPatientMeasurementsItem>>,
    name: Option<PreviewOrderResponseOrderInputPatientName>,
    phone: Option<String>,
    programs: Option<Vec<PreviewOrderResponseOrderInputPatientProgramsItem>>,
}

impl PreviewOrderResponseOrderInputPatientBuilder {
    pub fn address(mut self, value: PreviewOrderResponseOrderInputPatientAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn clinical_profile(
        mut self,
        value: PreviewOrderResponseOrderInputPatientClinicalProfile,
    ) -> Self {
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
        value: Vec<PreviewOrderResponseOrderInputPatientExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPatientAddressesItem>,
    ) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPatientEncountersItem>,
    ) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: PreviewOrderResponseOrderInputPatientGender) -> Self {
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

    pub fn measurements(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPatientMeasurementsItem>,
    ) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: PreviewOrderResponseOrderInputPatientName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn programs(
        mut self,
        value: Vec<PreviewOrderResponseOrderInputPatientProgramsItem>,
    ) -> Self {
        self.programs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPatient`].
    /// This method will fail if any of the following fields are not set:
    /// - [`date_of_birth`](PreviewOrderResponseOrderInputPatientBuilder::date_of_birth)
    /// - [`name`](PreviewOrderResponseOrderInputPatientBuilder::name)
    pub fn build(self) -> Result<PreviewOrderResponseOrderInputPatient, BuildError> {
        Ok(PreviewOrderResponseOrderInputPatient {
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
