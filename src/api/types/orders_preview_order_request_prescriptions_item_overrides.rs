pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequestPrescriptionsItemOverrides {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig: Option<PreviewOrderRequestPrescriptionsItemOverridesSig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<PreviewOrderRequestPrescriptionsItemOverridesQuantity>,
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refills: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<PreviewOrderRequestPrescriptionsItemOverridesClinical>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispensing: Option<PreviewOrderRequestPrescriptionsItemOverridesDispensing>,
}

impl PreviewOrderRequestPrescriptionsItemOverrides {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesBuilder {
    sig: Option<PreviewOrderRequestPrescriptionsItemOverridesSig>,
    quantity: Option<PreviewOrderRequestPrescriptionsItemOverridesQuantity>,
    days_supply: Option<i64>,
    refills: Option<i64>,
    clinical: Option<PreviewOrderRequestPrescriptionsItemOverridesClinical>,
    dispensing: Option<PreviewOrderRequestPrescriptionsItemOverridesDispensing>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesBuilder {
    pub fn sig(mut self, value: PreviewOrderRequestPrescriptionsItemOverridesSig) -> Self {
        self.sig = Some(value);
        self
    }

    pub fn quantity(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesQuantity,
    ) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn refills(mut self, value: i64) -> Self {
        self.refills = Some(value);
        self
    }

    pub fn clinical(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesClinical,
    ) -> Self {
        self.clinical = Some(value);
        self
    }

    pub fn dispensing(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesDispensing,
    ) -> Self {
        self.dispensing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverrides`].
    pub fn build(self) -> Result<PreviewOrderRequestPrescriptionsItemOverrides, BuildError> {
        Ok(PreviewOrderRequestPrescriptionsItemOverrides {
            sig: self.sig,
            quantity: self.quantity,
            days_supply: self.days_supply,
            refills: self.refills,
            clinical: self.clinical,
            dispensing: self.dispensing,
        })
    }
}
