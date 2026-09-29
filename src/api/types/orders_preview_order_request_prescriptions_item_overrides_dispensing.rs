pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesDispensing {
    #[serde(rename = "dispenseUponAcceptance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispense_upon_acceptance: Option<bool>,
    #[serde(rename = "shippingOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    /// Reviewed customer shipping rate for the selected service. Preview supplies this value. Shared group rates must not be summed per prescription.
    #[serde(rename = "shippingAmountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount_cents: Option<i64>,
    #[serde(rename = "shippingDestinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_destination_type:
        Option<PreviewOrderRequestPrescriptionsItemOverridesDispensingShippingDestinationType>,
    #[serde(rename = "pharmacyNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_notes: Option<String>,
    #[serde(rename = "requestedFillDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_fill_date: Option<String>,
    #[serde(rename = "substitutionPermitted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitution_permitted: Option<bool>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesDispensing {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesDispensingBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesDispensingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesDispensingBuilder {
    dispense_upon_acceptance: Option<bool>,
    shipping_option_id: Option<String>,
    shipping_amount_cents: Option<i64>,
    shipping_destination_type:
        Option<PreviewOrderRequestPrescriptionsItemOverridesDispensingShippingDestinationType>,
    pharmacy_notes: Option<String>,
    requested_fill_date: Option<String>,
    substitution_permitted: Option<bool>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesDispensingBuilder {
    pub fn dispense_upon_acceptance(mut self, value: bool) -> Self {
        self.dispense_upon_acceptance = Some(value);
        self
    }

    pub fn shipping_option_id(mut self, value: impl Into<String>) -> Self {
        self.shipping_option_id = Some(value.into());
        self
    }

    pub fn shipping_amount_cents(mut self, value: i64) -> Self {
        self.shipping_amount_cents = Some(value);
        self
    }

    pub fn shipping_destination_type(
        mut self,
        value: PreviewOrderRequestPrescriptionsItemOverridesDispensingShippingDestinationType,
    ) -> Self {
        self.shipping_destination_type = Some(value);
        self
    }

    pub fn pharmacy_notes(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_notes = Some(value.into());
        self
    }

    pub fn requested_fill_date(mut self, value: impl Into<String>) -> Self {
        self.requested_fill_date = Some(value.into());
        self
    }

    pub fn substitution_permitted(mut self, value: bool) -> Self {
        self.substitution_permitted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesDispensing`].
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesDispensing, BuildError> {
        Ok(PreviewOrderRequestPrescriptionsItemOverridesDispensing {
            dispense_upon_acceptance: self.dispense_upon_acceptance,
            shipping_option_id: self.shipping_option_id,
            shipping_amount_cents: self.shipping_amount_cents,
            shipping_destination_type: self.shipping_destination_type,
            pharmacy_notes: self.pharmacy_notes,
            requested_fill_date: self.requested_fill_date,
            substitution_permitted: self.substitution_permitted,
        })
    }
}
