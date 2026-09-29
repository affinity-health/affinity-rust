pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrdersResponseDataItemPrescriptionsItemProviderSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    pub legal_name: String,
    #[serde(rename = "licenseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_number: Option<String>,
    #[serde(rename = "licenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_state: Option<String>,
    #[serde(rename = "licenseExpiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_expires_at: Option<String>,
    #[serde(default)]
    pub npi: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl ListOrdersResponseDataItemPrescriptionsItemProviderSnapshot {
    pub fn builder() -> ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder {
        <ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder {
    address: Option<HashMap<String, serde_json::Value>>,
    credentials: Option<String>,
    legal_name: Option<String>,
    license_number: Option<String>,
    license_state: Option<String>,
    license_expires_at: Option<String>,
    npi: Option<String>,
    phone: Option<String>,
}

impl ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder {
    pub fn address(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.address = Some(value);
        self
    }

    pub fn credentials(mut self, value: impl Into<String>) -> Self {
        self.credentials = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn license_number(mut self, value: impl Into<String>) -> Self {
        self.license_number = Some(value.into());
        self
    }

    pub fn license_state(mut self, value: impl Into<String>) -> Self {
        self.license_state = Some(value.into());
        self
    }

    pub fn license_expires_at(mut self, value: impl Into<String>) -> Self {
        self.license_expires_at = Some(value.into());
        self
    }

    pub fn npi(mut self, value: impl Into<String>) -> Self {
        self.npi = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemPrescriptionsItemProviderSnapshot`].
    /// This method will fail if any of the following fields are not set:
    /// - [`legal_name`](ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder::legal_name)
    /// - [`npi`](ListOrdersResponseDataItemPrescriptionsItemProviderSnapshotBuilder::npi)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemPrescriptionsItemProviderSnapshot, BuildError> {
        Ok(
            ListOrdersResponseDataItemPrescriptionsItemProviderSnapshot {
                address: self.address,
                credentials: self.credentials,
                legal_name: self
                    .legal_name
                    .ok_or_else(|| BuildError::missing_field("legal_name"))?,
                license_number: self.license_number,
                license_state: self.license_state,
                license_expires_at: self.license_expires_at,
                npi: self.npi.ok_or_else(|| BuildError::missing_field("npi"))?,
                phone: self.phone,
            },
        )
    }
}
