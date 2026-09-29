pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOrderResponsePrescriptionsItemClinicalMedicationsItem {
    #[serde(default)]
    pub display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndc: Option<String>,
    #[serde(rename = "rxNormCui")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rx_norm_cui: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalMedicationsItem {
    pub fn builder() -> GetOrderResponsePrescriptionsItemClinicalMedicationsItemBuilder {
        <GetOrderResponsePrescriptionsItemClinicalMedicationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemClinicalMedicationsItemBuilder {
    display: Option<String>,
    ndc: Option<String>,
    rx_norm_cui: Option<String>,
    source: Option<String>,
}

impl GetOrderResponsePrescriptionsItemClinicalMedicationsItemBuilder {
    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    pub fn ndc(mut self, value: impl Into<String>) -> Self {
        self.ndc = Some(value.into());
        self
    }

    pub fn rx_norm_cui(mut self, value: impl Into<String>) -> Self {
        self.rx_norm_cui = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItemClinicalMedicationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](GetOrderResponsePrescriptionsItemClinicalMedicationsItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<GetOrderResponsePrescriptionsItemClinicalMedicationsItem, BuildError> {
        Ok(GetOrderResponsePrescriptionsItemClinicalMedicationsItem {
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
            ndc: self.ndc,
            rx_norm_cui: self.rx_norm_cui,
            source: self.source,
        })
    }
}
