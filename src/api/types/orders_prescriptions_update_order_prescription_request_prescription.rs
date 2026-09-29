pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateOrderPrescriptionRequestPrescription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<UpdateOrderPrescriptionRequestPrescriptionClinical>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "daysSupply")]
    #[serde(default)]
    pub days_supply: i64,
    #[serde(default)]
    pub dispensing: UpdateOrderPrescriptionRequestPrescriptionDispensing,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    pub quantity: UpdateOrderPrescriptionRequestPrescriptionQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<UpdateOrderPrescriptionRequestPrescriptionStructuredSig>,
}

impl UpdateOrderPrescriptionRequestPrescription {
    pub fn builder() -> UpdateOrderPrescriptionRequestPrescriptionBuilder {
        <UpdateOrderPrescriptionRequestPrescriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateOrderPrescriptionRequestPrescriptionBuilder {
    clinical: Option<UpdateOrderPrescriptionRequestPrescriptionClinical>,
    pharmacy_id: Option<String>,
    days_supply: Option<i64>,
    dispensing: Option<UpdateOrderPrescriptionRequestPrescriptionDispensing>,
    directions: Option<String>,
    medication_id: Option<String>,
    quantity: Option<UpdateOrderPrescriptionRequestPrescriptionQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    structured_sig: Option<UpdateOrderPrescriptionRequestPrescriptionStructuredSig>,
}

impl UpdateOrderPrescriptionRequestPrescriptionBuilder {
    pub fn clinical(mut self, value: UpdateOrderPrescriptionRequestPrescriptionClinical) -> Self {
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
        value: UpdateOrderPrescriptionRequestPrescriptionDispensing,
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

    pub fn quantity(mut self, value: UpdateOrderPrescriptionRequestPrescriptionQuantity) -> Self {
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
        value: UpdateOrderPrescriptionRequestPrescriptionStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateOrderPrescriptionRequestPrescription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days_supply`](UpdateOrderPrescriptionRequestPrescriptionBuilder::days_supply)
    /// - [`dispensing`](UpdateOrderPrescriptionRequestPrescriptionBuilder::dispensing)
    /// - [`directions`](UpdateOrderPrescriptionRequestPrescriptionBuilder::directions)
    /// - [`medication_id`](UpdateOrderPrescriptionRequestPrescriptionBuilder::medication_id)
    /// - [`quantity`](UpdateOrderPrescriptionRequestPrescriptionBuilder::quantity)
    /// - [`quantity_unit`](UpdateOrderPrescriptionRequestPrescriptionBuilder::quantity_unit)
    /// - [`refills`](UpdateOrderPrescriptionRequestPrescriptionBuilder::refills)
    pub fn build(self) -> Result<UpdateOrderPrescriptionRequestPrescription, BuildError> {
        Ok(UpdateOrderPrescriptionRequestPrescription {
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
