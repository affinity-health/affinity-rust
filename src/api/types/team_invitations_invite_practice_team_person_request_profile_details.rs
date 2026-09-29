pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvitePracticeTeamPersonRequestProfileDetails {
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "namePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "nameSuffix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialties: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsSpecialtiesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsAddressesItem>>,
    #[serde(rename = "otherNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_names: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certifications:
        Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsCertificationsItem>>,
}

impl InvitePracticeTeamPersonRequestProfileDetails {
    pub fn builder() -> InvitePracticeTeamPersonRequestProfileDetailsBuilder {
        <InvitePracticeTeamPersonRequestProfileDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvitePracticeTeamPersonRequestProfileDetailsBuilder {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    name_prefix: Option<String>,
    name_suffix: Option<String>,
    fax: Option<String>,
    specialties: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsSpecialtiesItem>>,
    addresses: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsAddressesItem>>,
    other_names: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem>>,
    identifiers: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem>>,
    endpoints: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem>>,
    certifications: Option<Vec<InvitePracticeTeamPersonRequestProfileDetailsCertificationsItem>>,
}

impl InvitePracticeTeamPersonRequestProfileDetailsBuilder {
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn name_prefix(mut self, value: impl Into<String>) -> Self {
        self.name_prefix = Some(value.into());
        self
    }

    pub fn name_suffix(mut self, value: impl Into<String>) -> Self {
        self.name_suffix = Some(value.into());
        self
    }

    pub fn fax(mut self, value: impl Into<String>) -> Self {
        self.fax = Some(value.into());
        self
    }

    pub fn specialties(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsSpecialtiesItem>,
    ) -> Self {
        self.specialties = Some(value);
        self
    }

    pub fn addresses(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsAddressesItem>,
    ) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn other_names(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsOtherNamesItem>,
    ) -> Self {
        self.other_names = Some(value);
        self
    }

    pub fn identifiers(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsIdentifiersItem>,
    ) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn endpoints(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsEndpointsItem>,
    ) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn certifications(
        mut self,
        value: Vec<InvitePracticeTeamPersonRequestProfileDetailsCertificationsItem>,
    ) -> Self {
        self.certifications = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvitePracticeTeamPersonRequestProfileDetails`].
    pub fn build(self) -> Result<InvitePracticeTeamPersonRequestProfileDetails, BuildError> {
        Ok(InvitePracticeTeamPersonRequestProfileDetails {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            name_prefix: self.name_prefix,
            name_suffix: self.name_suffix,
            fax: self.fax,
            specialties: self.specialties,
            addresses: self.addresses,
            other_names: self.other_names,
            identifiers: self.identifiers,
            endpoints: self.endpoints,
            certifications: self.certifications,
        })
    }
}
