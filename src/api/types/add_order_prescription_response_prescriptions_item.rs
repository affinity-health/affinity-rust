pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddOrderPrescriptionResponsePrescriptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(default)]
    pub version: i64,
}

impl AddOrderPrescriptionResponsePrescriptionsItem {
    pub fn builder() -> AddOrderPrescriptionResponsePrescriptionsItemBuilder {
        <AddOrderPrescriptionResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionResponsePrescriptionsItemBuilder {
    id: Option<String>,
    external_prescription_id: Option<String>,
    version: Option<i64>,
}

impl AddOrderPrescriptionResponsePrescriptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddOrderPrescriptionResponsePrescriptionsItemBuilder::id)
    /// - [`version`](AddOrderPrescriptionResponsePrescriptionsItemBuilder::version)
    pub fn build(self) -> Result<AddOrderPrescriptionResponsePrescriptionsItem, BuildError> {
        Ok(AddOrderPrescriptionResponsePrescriptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_prescription_id: self.external_prescription_id,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
