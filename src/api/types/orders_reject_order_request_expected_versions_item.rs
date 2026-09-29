pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RejectOrderRequestExpectedVersionsItem {
    #[serde(rename = "prescriptionId")]
    #[serde(default)]
    pub prescription_id: String,
    #[serde(default)]
    pub version: i64,
}

impl RejectOrderRequestExpectedVersionsItem {
    pub fn builder() -> RejectOrderRequestExpectedVersionsItemBuilder {
        <RejectOrderRequestExpectedVersionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RejectOrderRequestExpectedVersionsItemBuilder {
    prescription_id: Option<String>,
    version: Option<i64>,
}

impl RejectOrderRequestExpectedVersionsItemBuilder {
    pub fn prescription_id(mut self, value: impl Into<String>) -> Self {
        self.prescription_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RejectOrderRequestExpectedVersionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prescription_id`](RejectOrderRequestExpectedVersionsItemBuilder::prescription_id)
    /// - [`version`](RejectOrderRequestExpectedVersionsItemBuilder::version)
    pub fn build(self) -> Result<RejectOrderRequestExpectedVersionsItem, BuildError> {
        Ok(RejectOrderRequestExpectedVersionsItem {
            prescription_id: self
                .prescription_id
                .ok_or_else(|| BuildError::missing_field("prescription_id"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
