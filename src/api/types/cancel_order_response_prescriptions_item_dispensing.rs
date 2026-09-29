pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelOrderResponsePrescriptionsItemDispensing {
    #[serde(rename = "dispenseUponAcceptance")]
    #[serde(default)]
    pub dispense_upon_acceptance: bool,
    #[serde(rename = "substitutionPermitted")]
    #[serde(default)]
    pub substitution_permitted: bool,
    #[serde(rename = "pharmacyNotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_notes: Option<String>,
    #[serde(rename = "requestedFillDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_fill_date: Option<String>,
    #[serde(rename = "shippingOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(rename = "shippingAmountCents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_amount_cents: Option<i64>,
    #[serde(rename = "shippingDestinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_destination_type:
        Option<CancelOrderResponsePrescriptionsItemDispensingShippingDestinationType>,
}

impl CancelOrderResponsePrescriptionsItemDispensing {
    pub fn builder() -> CancelOrderResponsePrescriptionsItemDispensingBuilder {
        <CancelOrderResponsePrescriptionsItemDispensingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelOrderResponsePrescriptionsItemDispensingBuilder {
    dispense_upon_acceptance: Option<bool>,
    substitution_permitted: Option<bool>,
    pharmacy_notes: Option<String>,
    requested_fill_date: Option<String>,
    shipping_option_id: Option<String>,
    shipping_amount_cents: Option<i64>,
    shipping_destination_type:
        Option<CancelOrderResponsePrescriptionsItemDispensingShippingDestinationType>,
}

impl CancelOrderResponsePrescriptionsItemDispensingBuilder {
    pub fn dispense_upon_acceptance(mut self, value: bool) -> Self {
        self.dispense_upon_acceptance = Some(value);
        self
    }

    pub fn substitution_permitted(mut self, value: bool) -> Self {
        self.substitution_permitted = Some(value);
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
        value: CancelOrderResponsePrescriptionsItemDispensingShippingDestinationType,
    ) -> Self {
        self.shipping_destination_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CancelOrderResponsePrescriptionsItemDispensing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dispense_upon_acceptance`](CancelOrderResponsePrescriptionsItemDispensingBuilder::dispense_upon_acceptance)
    /// - [`substitution_permitted`](CancelOrderResponsePrescriptionsItemDispensingBuilder::substitution_permitted)
    pub fn build(self) -> Result<CancelOrderResponsePrescriptionsItemDispensing, BuildError> {
        Ok(CancelOrderResponsePrescriptionsItemDispensing {
            dispense_upon_acceptance: self
                .dispense_upon_acceptance
                .ok_or_else(|| BuildError::missing_field("dispense_upon_acceptance"))?,
            substitution_permitted: self
                .substitution_permitted
                .ok_or_else(|| BuildError::missing_field("substitution_permitted"))?,
            pharmacy_notes: self.pharmacy_notes,
            requested_fill_date: self.requested_fill_date,
            shipping_option_id: self.shipping_option_id,
            shipping_amount_cents: self.shipping_amount_cents,
            shipping_destination_type: self.shipping_destination_type,
        })
    }
}
