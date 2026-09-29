pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddOrderPrescriptionRequestPrescription {
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<AddOrderPrescriptionRequestPrescriptionClinical>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "daysSupply")]
    #[serde(default)]
    pub days_supply: i64,
    #[serde(default)]
    pub dispensing: AddOrderPrescriptionRequestPrescriptionDispensing,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    pub quantity: AddOrderPrescriptionRequestPrescriptionQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<AddOrderPrescriptionRequestPrescriptionStructuredSig>,
}

impl AddOrderPrescriptionRequestPrescription {
    pub fn builder() -> AddOrderPrescriptionRequestPrescriptionBuilder {
        <AddOrderPrescriptionRequestPrescriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrderPrescriptionRequestPrescriptionBuilder {
    external_prescription_id: Option<String>,
    clinical: Option<AddOrderPrescriptionRequestPrescriptionClinical>,
    pharmacy_id: Option<String>,
    days_supply: Option<i64>,
    dispensing: Option<AddOrderPrescriptionRequestPrescriptionDispensing>,
    directions: Option<String>,
    medication_id: Option<String>,
    quantity: Option<AddOrderPrescriptionRequestPrescriptionQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    structured_sig: Option<AddOrderPrescriptionRequestPrescriptionStructuredSig>,
}

impl AddOrderPrescriptionRequestPrescriptionBuilder {
    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn clinical(mut self, value: AddOrderPrescriptionRequestPrescriptionClinical) -> Self {
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

    pub fn dispensing(mut self, value: AddOrderPrescriptionRequestPrescriptionDispensing) -> Self {
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

    pub fn quantity(mut self, value: AddOrderPrescriptionRequestPrescriptionQuantity) -> Self {
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
        value: AddOrderPrescriptionRequestPrescriptionStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrderPrescriptionRequestPrescription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days_supply`](AddOrderPrescriptionRequestPrescriptionBuilder::days_supply)
    /// - [`dispensing`](AddOrderPrescriptionRequestPrescriptionBuilder::dispensing)
    /// - [`directions`](AddOrderPrescriptionRequestPrescriptionBuilder::directions)
    /// - [`medication_id`](AddOrderPrescriptionRequestPrescriptionBuilder::medication_id)
    /// - [`quantity`](AddOrderPrescriptionRequestPrescriptionBuilder::quantity)
    /// - [`quantity_unit`](AddOrderPrescriptionRequestPrescriptionBuilder::quantity_unit)
    /// - [`refills`](AddOrderPrescriptionRequestPrescriptionBuilder::refills)
    pub fn build(self) -> Result<AddOrderPrescriptionRequestPrescription, BuildError> {
        Ok(AddOrderPrescriptionRequestPrescription {
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
