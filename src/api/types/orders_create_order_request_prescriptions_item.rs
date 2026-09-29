pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderRequestPrescriptionsItem {
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<CreateOrderRequestPrescriptionsItemClinical>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "daysSupply")]
    #[serde(default)]
    pub days_supply: i64,
    #[serde(default)]
    pub dispensing: CreateOrderRequestPrescriptionsItemDispensing,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    pub quantity: CreateOrderRequestPrescriptionsItemQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<CreateOrderRequestPrescriptionsItemStructuredSig>,
}

impl CreateOrderRequestPrescriptionsItem {
    pub fn builder() -> CreateOrderRequestPrescriptionsItemBuilder {
        <CreateOrderRequestPrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriptionsItemBuilder {
    external_prescription_id: Option<String>,
    clinical: Option<CreateOrderRequestPrescriptionsItemClinical>,
    pharmacy_id: Option<String>,
    days_supply: Option<i64>,
    dispensing: Option<CreateOrderRequestPrescriptionsItemDispensing>,
    directions: Option<String>,
    medication_id: Option<String>,
    quantity: Option<CreateOrderRequestPrescriptionsItemQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    structured_sig: Option<CreateOrderRequestPrescriptionsItemStructuredSig>,
}

impl CreateOrderRequestPrescriptionsItemBuilder {
    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn clinical(mut self, value: CreateOrderRequestPrescriptionsItemClinical) -> Self {
        self.clinical = Some(value);
        self
    }

    pub fn pharmacy_id(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_id = Some(value.into());
        self
    }

    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn dispensing(mut self, value: CreateOrderRequestPrescriptionsItemDispensing) -> Self {
        self.dispensing = Some(value);
        self
    }

    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn medication_id(mut self, value: impl Into<String>) -> Self {
        self.medication_id = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: CreateOrderRequestPrescriptionsItemQuantity) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn quantity_unit(mut self, value: impl Into<String>) -> Self {
        self.quantity_unit = Some(value.into());
        self
    }

    pub fn refills(mut self, value: i64) -> Self {
        self.refills = Some(value);
        self
    }

    pub fn structured_sig(
        mut self,
        value: CreateOrderRequestPrescriptionsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days_supply`](CreateOrderRequestPrescriptionsItemBuilder::days_supply)
    /// - [`dispensing`](CreateOrderRequestPrescriptionsItemBuilder::dispensing)
    /// - [`directions`](CreateOrderRequestPrescriptionsItemBuilder::directions)
    /// - [`medication_id`](CreateOrderRequestPrescriptionsItemBuilder::medication_id)
    /// - [`quantity`](CreateOrderRequestPrescriptionsItemBuilder::quantity)
    /// - [`quantity_unit`](CreateOrderRequestPrescriptionsItemBuilder::quantity_unit)
    /// - [`refills`](CreateOrderRequestPrescriptionsItemBuilder::refills)
    pub fn build(self) -> Result<CreateOrderRequestPrescriptionsItem, BuildError> {
        Ok(CreateOrderRequestPrescriptionsItem {
            external_prescription_id: self.external_prescription_id,
            clinical: self.clinical,
            pharmacy_id: self.pharmacy_id,
            days_supply: self
                .days_supply
                .ok_or_else(|| BuildError::missing_field("days_supply"))?,
            dispensing: self
                .dispensing
                .ok_or_else(|| BuildError::missing_field("dispensing"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            medication_id: self
                .medication_id
                .ok_or_else(|| BuildError::missing_field("medication_id"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            quantity_unit: self
                .quantity_unit
                .ok_or_else(|| BuildError::missing_field("quantity_unit"))?,
            refills: self
                .refills
                .ok_or_else(|| BuildError::missing_field("refills"))?,
            structured_sig: self.structured_sig,
        })
    }
}
