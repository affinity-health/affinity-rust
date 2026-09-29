pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderPrescriptionResponsePrescriptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(default)]
    pub version: i64,
}

impl UpdateOrderPrescriptionResponsePrescriptionsItem {
    pub fn builder() -> UpdateOrderPrescriptionResponsePrescriptionsItemBuilder {
        <UpdateOrderPrescriptionResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionResponsePrescriptionsItemBuilder {
    id: Option<String>,
    external_prescription_id: Option<String>,
    version: Option<i64>,
}

impl UpdateOrderPrescriptionResponsePrescriptionsItemBuilder {
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

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateOrderPrescriptionResponsePrescriptionsItemBuilder::id)
    /// - [`version`](UpdateOrderPrescriptionResponsePrescriptionsItemBuilder::version)
    pub fn build(self) -> Result<UpdateOrderPrescriptionResponsePrescriptionsItem, BuildError> {
        Ok(UpdateOrderPrescriptionResponsePrescriptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            external_prescription_id: self.external_prescription_id,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
