pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem {
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

impl ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem {
    pub fn builder() -> ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItemBuilder {
        <ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItemBuilder {
    display: Option<String>,
    ndc: Option<String>,
    rx_norm_cui: Option<String>,
    source: Option<String>,
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItemBuilder {
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

    /// Consumes the builder and constructs a [`ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display`](ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem, BuildError>
    {
        Ok(
            ListOrdersResponseDataItemPrescriptionsItemClinicalMedicationsItem {
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
                ndc: self.ndc,
                rx_norm_cui: self.rx_norm_cui,
                source: self.source,
            },
        )
    }
}
