pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponseOrderInputPrescriptionsItem {
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<PreviewOrderResponseOrderInputPrescriptionsItemClinical>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "daysSupply")]
    #[serde(default)]
    pub days_supply: i64,
    #[serde(default)]
    pub dispensing: PreviewOrderResponseOrderInputPrescriptionsItemDispensing,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "medicationId")]
    #[serde(default)]
    pub medication_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<PreviewOrderResponseOrderInputPrescriptionsItemStructuredSig>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItem {
    pub fn builder() -> PreviewOrderResponseOrderInputPrescriptionsItemBuilder {
        <PreviewOrderResponseOrderInputPrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemBuilder {
    external_prescription_id: Option<String>,
    clinical: Option<PreviewOrderResponseOrderInputPrescriptionsItemClinical>,
    pharmacy_id: Option<String>,
    days_supply: Option<i64>,
    dispensing: Option<PreviewOrderResponseOrderInputPrescriptionsItemDispensing>,
    directions: Option<String>,
    medication_id: Option<String>,
    quantity: Option<f64>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    structured_sig: Option<PreviewOrderResponseOrderInputPrescriptionsItemStructuredSig>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemBuilder {
    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn clinical(
        mut self,
        value: PreviewOrderResponseOrderInputPrescriptionsItemClinical,
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
        value: PreviewOrderResponseOrderInputPrescriptionsItemDispensing,
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

    pub fn quantity(mut self, value: f64) -> Self {
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
        value: PreviewOrderResponseOrderInputPrescriptionsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`days_supply`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::days_supply)
    /// - [`dispensing`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::dispensing)
    /// - [`directions`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::directions)
    /// - [`medication_id`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::medication_id)
    /// - [`quantity`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::quantity)
    /// - [`quantity_unit`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::quantity_unit)
    /// - [`refills`](PreviewOrderResponseOrderInputPrescriptionsItemBuilder::refills)
    pub fn build(self) -> Result<PreviewOrderResponseOrderInputPrescriptionsItem, BuildError> {
        Ok(PreviewOrderResponseOrderInputPrescriptionsItem {
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
