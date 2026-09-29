pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePatientRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdatePatientRequestAddress>,
    #[serde(rename = "clinicalProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_profile: Option<UpdatePatientRequestClinicalProfile>,
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "externalIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_identities: Option<Vec<UpdatePatientRequestExternalIdentitiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<UpdatePatientRequestAddressesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encounters: Option<Vec<UpdatePatientRequestEncountersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<UpdatePatientRequestGender>,
    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "medicalRecordNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_record_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<Vec<UpdatePatientRequestMeasurementsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdatePatientRequestName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programs: Option<Vec<UpdatePatientRequestProgramsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdatePatientRequestStatus>,
}

impl UpdatePatientRequest {
    pub fn builder() -> UpdatePatientRequestBuilder {
        <UpdatePatientRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePatientRequestBuilder {
    address: Option<UpdatePatientRequestAddress>,
    clinical_profile: Option<UpdatePatientRequestClinicalProfile>,
    date_of_birth: Option<String>,
    email: Option<String>,
    external_id: Option<String>,
    external_identities: Option<Vec<UpdatePatientRequestExternalIdentitiesItem>>,
    addresses: Option<Vec<UpdatePatientRequestAddressesItem>>,
    encounters: Option<Vec<UpdatePatientRequestEncountersItem>>,
    gender: Option<UpdatePatientRequestGender>,
    location_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    medical_record_number: Option<String>,
    measurements: Option<Vec<UpdatePatientRequestMeasurementsItem>>,
    name: Option<UpdatePatientRequestName>,
    programs: Option<Vec<UpdatePatientRequestProgramsItem>>,
    phone: Option<String>,
    status: Option<UpdatePatientRequestStatus>,
}

impl UpdatePatientRequestBuilder {
    pub fn address(mut self, value: UpdatePatientRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn clinical_profile(mut self, value: UpdatePatientRequestClinicalProfile) -> Self {
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
        value: Vec<UpdatePatientRequestExternalIdentitiesItem>,
    ) -> Self {
        self.external_identities = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<UpdatePatientRequestAddressesItem>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn encounters(mut self, value: Vec<UpdatePatientRequestEncountersItem>) -> Self {
        self.encounters = Some(value);
        self
    }

    pub fn gender(mut self, value: UpdatePatientRequestGender) -> Self {
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

    pub fn measurements(mut self, value: Vec<UpdatePatientRequestMeasurementsItem>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn name(mut self, value: UpdatePatientRequestName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn programs(mut self, value: Vec<UpdatePatientRequestProgramsItem>) -> Self {
        self.programs = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn status(mut self, value: UpdatePatientRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePatientRequest`].
    pub fn build(self) -> Result<UpdatePatientRequest, BuildError> {
        Ok(UpdatePatientRequest {
            address: self.address,
            clinical_profile: self.clinical_profile,
            date_of_birth: self.date_of_birth,
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
            name: self.name,
            programs: self.programs,
            phone: self.phone,
            status: self.status,
        })
    }
}
