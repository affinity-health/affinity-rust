pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetOrderResponsePrescriptionsItem {
    #[serde(default)]
    pub version: i64,
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<GetOrderResponsePrescriptionsItemDaysSupply>,
    #[serde(rename = "patientSnapshot")]
    #[serde(default)]
    pub patient_snapshot: GetOrderResponsePrescriptionsItemPatientSnapshot,
    /// The saved delivery address for this prescription version. Patient profile updates do not replace it. Review this address before signing.
    #[serde(rename = "deliveryAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_address: Option<HashMap<String, serde_json::Value>>,
    /// Whether the saved delivery address differs from the current primary patient address. This can be intentional; confirm the delivery address before signing.
    #[serde(rename = "deliveryAddressDiffersFromPatient")]
    #[serde(default)]
    pub delivery_address_differs_from_patient: bool,
    #[serde(rename = "providerSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_snapshot: Option<GetOrderResponsePrescriptionsItemProviderSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical: Option<GetOrderResponsePrescriptionsItemClinical>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispensing: Option<GetOrderResponsePrescriptionsItemDispensing>,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<GetOrderResponsePrescriptionsItemStructuredSig>,
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(rename = "catalogItemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[serde(rename = "pharmacyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_id: Option<String>,
    #[serde(rename = "pharmacyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pharmacy_name: Option<String>,
    #[serde(default)]
    pub directions: String,
    #[serde(rename = "dosageForm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage_form: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "medicationName")]
    #[serde(default)]
    pub medication_name: String,
    pub quantity: GetOrderResponsePrescriptionsItemQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    #[serde(default)]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
}

impl GetOrderResponsePrescriptionsItem {
    pub fn builder() -> GetOrderResponsePrescriptionsItemBuilder {
        <GetOrderResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOrderResponsePrescriptionsItemBuilder {
    version: Option<i64>,
    days_supply: Option<GetOrderResponsePrescriptionsItemDaysSupply>,
    patient_snapshot: Option<GetOrderResponsePrescriptionsItemPatientSnapshot>,
    delivery_address: Option<HashMap<String, serde_json::Value>>,
    delivery_address_differs_from_patient: Option<bool>,
    provider_snapshot: Option<GetOrderResponsePrescriptionsItemProviderSnapshot>,
    clinical: Option<GetOrderResponsePrescriptionsItemClinical>,
    dispensing: Option<GetOrderResponsePrescriptionsItemDispensing>,
    structured_sig: Option<GetOrderResponsePrescriptionsItemStructuredSig>,
    external_prescription_id: Option<String>,
    catalog_item_id: Option<String>,
    pharmacy_id: Option<String>,
    pharmacy_name: Option<String>,
    directions: Option<String>,
    dosage_form: Option<String>,
    id: Option<String>,
    medication_name: Option<String>,
    quantity: Option<GetOrderResponsePrescriptionsItemQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    status: Option<String>,
    strength: Option<String>,
}

impl GetOrderResponsePrescriptionsItemBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn days_supply(mut self, value: GetOrderResponsePrescriptionsItemDaysSupply) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn patient_snapshot(
        mut self,
        value: GetOrderResponsePrescriptionsItemPatientSnapshot,
    ) -> Self {
        self.patient_snapshot = Some(value);
        self
    }

    pub fn delivery_address(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.delivery_address = Some(value);
        self
    }

    pub fn delivery_address_differs_from_patient(mut self, value: bool) -> Self {
        self.delivery_address_differs_from_patient = Some(value);
        self
    }

    pub fn provider_snapshot(
        mut self,
        value: GetOrderResponsePrescriptionsItemProviderSnapshot,
    ) -> Self {
        self.provider_snapshot = Some(value);
        self
    }

    pub fn clinical(mut self, value: GetOrderResponsePrescriptionsItemClinical) -> Self {
        self.clinical = Some(value);
        self
    }

    pub fn dispensing(mut self, value: GetOrderResponsePrescriptionsItemDispensing) -> Self {
        self.dispensing = Some(value);
        self
    }

    pub fn structured_sig(mut self, value: GetOrderResponsePrescriptionsItemStructuredSig) -> Self {
        self.structured_sig = Some(value);
        self
    }

    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_item_id = Some(value.into());
        self
    }

    pub fn pharmacy_id(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_id = Some(value.into());
        self
    }

    pub fn pharmacy_name(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_name = Some(value.into());
        self
    }

    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn dosage_form(mut self, value: impl Into<String>) -> Self {
        self.dosage_form = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn medication_name(mut self, value: impl Into<String>) -> Self {
        self.medication_name = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: GetOrderResponsePrescriptionsItemQuantity) -> Self {
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

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn strength(mut self, value: impl Into<String>) -> Self {
        self.strength = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetOrderResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](GetOrderResponsePrescriptionsItemBuilder::version)
    /// - [`patient_snapshot`](GetOrderResponsePrescriptionsItemBuilder::patient_snapshot)
    /// - [`delivery_address_differs_from_patient`](GetOrderResponsePrescriptionsItemBuilder::delivery_address_differs_from_patient)
    /// - [`directions`](GetOrderResponsePrescriptionsItemBuilder::directions)
    /// - [`id`](GetOrderResponsePrescriptionsItemBuilder::id)
    /// - [`medication_name`](GetOrderResponsePrescriptionsItemBuilder::medication_name)
    /// - [`quantity`](GetOrderResponsePrescriptionsItemBuilder::quantity)
    /// - [`quantity_unit`](GetOrderResponsePrescriptionsItemBuilder::quantity_unit)
    /// - [`refills`](GetOrderResponsePrescriptionsItemBuilder::refills)
    /// - [`status`](GetOrderResponsePrescriptionsItemBuilder::status)
    pub fn build(self) -> Result<GetOrderResponsePrescriptionsItem, BuildError> {
        Ok(GetOrderResponsePrescriptionsItem {
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            days_supply: self.days_supply,
            patient_snapshot: self
                .patient_snapshot
                .ok_or_else(|| BuildError::missing_field("patient_snapshot"))?,
            delivery_address: self.delivery_address,
            delivery_address_differs_from_patient: self
                .delivery_address_differs_from_patient
                .ok_or_else(|| {
                    BuildError::missing_field("delivery_address_differs_from_patient")
                })?,
            provider_snapshot: self.provider_snapshot,
            clinical: self.clinical,
            dispensing: self.dispensing,
            structured_sig: self.structured_sig,
            external_prescription_id: self.external_prescription_id,
            catalog_item_id: self.catalog_item_id,
            pharmacy_id: self.pharmacy_id,
            pharmacy_name: self.pharmacy_name,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            dosage_form: self.dosage_form,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            medication_name: self
                .medication_name
                .ok_or_else(|| BuildError::missing_field("medication_name"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            quantity_unit: self
                .quantity_unit
                .ok_or_else(|| BuildError::missing_field("quantity_unit"))?,
            refills: self
                .refills
                .ok_or_else(|| BuildError::missing_field("refills"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            strength: self.strength,
        })
    }
}
