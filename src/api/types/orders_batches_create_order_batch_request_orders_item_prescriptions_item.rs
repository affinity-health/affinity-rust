pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItem {
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "daysSupply")]
    #[serde(default)]
    pub days_supply: i64,
    #[serde(default)]
    pub dispensing: CreateOrderBatchRequestOrdersItemPrescriptionsItemDispensing,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    pub quantity: CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemStructuredSig>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItem {
    pub fn builder() -> CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder {
        <CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder {
    external_prescription_id: Option<String>,
    clinical: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical>,
    pharmacy_id: Option<String>,
    days_supply: Option<i64>,
    dispensing: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemDispensing>,
    directions: Option<String>,
    medication_id: Option<String>,
    quantity: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    structured_sig: Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemStructuredSig>,
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder {
    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn clinical(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemClinical,
    ) -> Self {
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

    pub fn dispensing(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemDispensing,
    ) -> Self {
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

    pub fn quantity(
        mut self,
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemQuantity,
    ) -> Self {
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
        value: CreateOrderBatchRequestOrdersItemPrescriptionsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderBatchRequestOrdersItemPrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days_supply`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::days_supply)
    /// - [`dispensing`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::dispensing)
    /// - [`directions`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::directions)
    /// - [`medication_id`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::medication_id)
    /// - [`quantity`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::quantity)
    /// - [`quantity_unit`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::quantity_unit)
    /// - [`refills`](CreateOrderBatchRequestOrdersItemPrescriptionsItemBuilder::refills)
    pub fn build(self) -> Result<CreateOrderBatchRequestOrdersItemPrescriptionsItem, BuildError> {
        Ok(CreateOrderBatchRequestOrdersItemPrescriptionsItem {
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
