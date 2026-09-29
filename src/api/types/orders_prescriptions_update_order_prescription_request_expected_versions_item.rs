pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateOrderPrescriptionRequestExpectedVersionsItem {
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub version: i64,
}

impl UpdateOrderPrescriptionRequestExpectedVersionsItem {
    pub fn builder() -> UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder {
        <UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder {
    prescription_id: Option<String>,
    version: Option<i64>,
}

impl UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder {
    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestExpectedVersionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prescription_id`](UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder::prescription_id)
    /// - [`version`](UpdateOrderPrescriptionRequestExpectedVersionsItemBuilder::version)
    pub fn build(self) -> Result<UpdateOrderPrescriptionRequestExpectedVersionsItem, BuildError> {
        Ok(UpdateOrderPrescriptionRequestExpectedVersionsItem {
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
