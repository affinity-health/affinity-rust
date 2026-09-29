pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateOrderResponsePrescriptionsItem {
    #[serde(rename = "pharmacyId")]
    #[serde(default)]
    pub pharmacy_id: String,
    #[serde(rename = "externalPrescriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_prescription_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub directions: String,
    #[serde(default)]
    pub version: i64,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "medicationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_id: Option<String>,
    #[serde(rename = "medicationName")]
    #[serde(default)]
    pub medication_name: String,
    pub object: CreateOrderResponsePrescriptionsItemObject,
    pub quantity: CreateOrderResponsePrescriptionsItemQuantity,
    #[serde(rename = "quantityUnit")]
    #[serde(default)]
    pub quantity_unit: String,
    #[serde(default)]
    pub refills: i64,
    pub status: CreateOrderResponsePrescriptionsItemStatus,
}

impl CreateOrderResponsePrescriptionsItem {
    pub fn builder() -> CreateOrderResponsePrescriptionsItemBuilder {
        <CreateOrderResponsePrescriptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderResponsePrescriptionsItemBuilder {
    pharmacy_id: Option<String>,
    external_prescription_id: Option<String>,
    created_at: Option<String>,
    directions: Option<String>,
    version: Option<i64>,
    id: Option<String>,
    medication_id: Option<String>,
    medication_name: Option<String>,
    object: Option<CreateOrderResponsePrescriptionsItemObject>,
    quantity: Option<CreateOrderResponsePrescriptionsItemQuantity>,
    quantity_unit: Option<String>,
    refills: Option<i64>,
    status: Option<CreateOrderResponsePrescriptionsItemStatus>,
}

impl CreateOrderResponsePrescriptionsItemBuilder {
    pub fn pharmacy_id(mut self, value: impl Into<String>) -> Self {
        self.pharmacy_id = Some(value.into());
        self
    }

    pub fn external_prescription_id(mut self, value: impl Into<String>) -> Self {
        self.external_prescription_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn medication_id(mut self, value: impl Into<String>) -> Self {
        self.medication_id = Some(value.into());
        self
    }

    pub fn medication_name(mut self, value: impl Into<String>) -> Self {
        self.medication_name = Some(value.into());
        self
    }

    pub fn object(mut self, value: CreateOrderResponsePrescriptionsItemObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn quantity(mut self, value: CreateOrderResponsePrescriptionsItemQuantity) -> Self {
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

    pub fn status(mut self, value: CreateOrderResponsePrescriptionsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderResponsePrescriptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pharmacy_id`](CreateOrderResponsePrescriptionsItemBuilder::pharmacy_id)
    /// - [`created_at`](CreateOrderResponsePrescriptionsItemBuilder::created_at)
    /// - [`directions`](CreateOrderResponsePrescriptionsItemBuilder::directions)
    /// - [`version`](CreateOrderResponsePrescriptionsItemBuilder::version)
    /// - [`id`](CreateOrderResponsePrescriptionsItemBuilder::id)
    /// - [`medication_name`](CreateOrderResponsePrescriptionsItemBuilder::medication_name)
    /// - [`object`](CreateOrderResponsePrescriptionsItemBuilder::object)
    /// - [`quantity`](CreateOrderResponsePrescriptionsItemBuilder::quantity)
    /// - [`quantity_unit`](CreateOrderResponsePrescriptionsItemBuilder::quantity_unit)
    /// - [`refills`](CreateOrderResponsePrescriptionsItemBuilder::refills)
    /// - [`status`](CreateOrderResponsePrescriptionsItemBuilder::status)
    pub fn build(self) -> Result<CreateOrderResponsePrescriptionsItem, BuildError> {
        Ok(CreateOrderResponsePrescriptionsItem {
            pharmacy_id: self
                .pharmacy_id
                .ok_or_else(|| BuildError::missing_field("pharmacy_id"))?,
            external_prescription_id: self.external_prescription_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            medication_id: self.medication_id,
            medication_name: self
                .medication_name
                .ok_or_else(|| BuildError::missing_field("medication_name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
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
        })
    }
}
