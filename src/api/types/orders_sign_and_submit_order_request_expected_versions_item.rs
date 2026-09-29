pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SignAndSubmitOrderRequestExpectedVersionsItem {
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub version: i64,
}

impl SignAndSubmitOrderRequestExpectedVersionsItem {
    pub fn builder() -> SignAndSubmitOrderRequestExpectedVersionsItemBuilder {
        <SignAndSubmitOrderRequestExpectedVersionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignAndSubmitOrderRequestExpectedVersionsItemBuilder {
    prescription_id: Option<String>,
    version: Option<i64>,
}

impl SignAndSubmitOrderRequestExpectedVersionsItemBuilder {
    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignAndSubmitOrderRequestExpectedVersionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prescription_id`](SignAndSubmitOrderRequestExpectedVersionsItemBuilder::prescription_id)
    /// - [`version`](SignAndSubmitOrderRequestExpectedVersionsItemBuilder::version)
    pub fn build(self) -> Result<SignAndSubmitOrderRequestExpectedVersionsItem, BuildError> {
        Ok(SignAndSubmitOrderRequestExpectedVersionsItem {
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
