pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalog {
    #[serde(rename = "catalogDetails")]
    #[serde(default)]
    pub catalog_details: RetrievePrescribingOptionsResponseCatalogCatalogDetails,
    pub composition: RetrievePrescribingOptionsResponseCatalogComposition,
    #[serde(rename = "allowedStates")]
    #[serde(default)]
    pub allowed_states: Vec<String>,
    pub availability: RetrievePrescribingOptionsResponseCatalogAvailability,
    #[serde(rename = "catalogKind")]
    #[serde(default)]
    pub catalog_kind: String,
    #[serde(rename = "fulfillmentInclusions")]
    #[serde(default)]
    pub fulfillment_inclusions:
        Vec<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem>,
    pub ordering: RetrievePrescribingOptionsResponseCatalogOrdering,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "coldShip")]
    #[serde(default)]
    pub cold_ship: bool,
    #[serde(rename = "pharmacyId")]
    #[serde(default)]
    pub pharmacy_id: String,
    #[serde(rename = "pharmacyName")]
    #[serde(default)]
    pub pharmacy_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "dosageForm")]
    #[serde(default)]
    pub dosage_form: String,
    #[serde(rename = "facilityType")]
    #[serde(default)]
    pub facility_type: String,
    #[serde(default)]
    pub id: String,
    /// Primary product photo, falling back to dosage-form artwork. Null when neither is available.
    #[serde(rename = "imageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "imageUrls")]
    #[serde(default)]
    pub image_urls: Vec<String>,
    #[serde(rename = "medicationGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_group: Option<RetrievePrescribingOptionsResponseCatalogMedicationGroup>,
    #[serde(rename = "isOrderable")]
    #[serde(default)]
    pub is_orderable: bool,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub name: String,
    pub object: RetrievePrescribingOptionsResponseCatalogObject,
    #[serde(rename = "patientSpecificRequired")]
    #[serde(default)]
    pub patient_specific_required: bool,
    #[serde(rename = "quantityConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_constraint: Option<RetrievePrescribingOptionsResponseCatalogQuantityConstraint>,
    #[serde(rename = "prescriptionRequirements")]
    pub prescription_requirements:
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing: Option<RetrievePrescribingOptionsResponseCatalogPricing>,
    #[serde(rename = "restrictedStates")]
    #[serde(default)]
    pub restricted_states: Vec<String>,
    #[serde(default)]
    pub route: String,
    #[serde(rename = "shippingOptions")]
    #[serde(default)]
    pub shipping_options: Vec<RetrievePrescribingOptionsResponseCatalogShippingOptionsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalog {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogBuilder {
        <RetrievePrescribingOptionsResponseCatalogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogBuilder {
    catalog_details: Option<RetrievePrescribingOptionsResponseCatalogCatalogDetails>,
    composition: Option<RetrievePrescribingOptionsResponseCatalogComposition>,
    allowed_states: Option<Vec<String>>,
    availability: Option<RetrievePrescribingOptionsResponseCatalogAvailability>,
    catalog_kind: Option<String>,
    fulfillment_inclusions:
        Option<Vec<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem>>,
    ordering: Option<RetrievePrescribingOptionsResponseCatalogOrdering>,
    category: Option<String>,
    cold_ship: Option<bool>,
    pharmacy_id: Option<String>,
    pharmacy_name: Option<String>,
    description: Option<String>,
    dosage_form: Option<String>,
    facility_type: Option<String>,
    id: Option<String>,
    image_url: Option<String>,
    image_urls: Option<Vec<String>>,
    medication_group: Option<RetrievePrescribingOptionsResponseCatalogMedicationGroup>,
    is_orderable: Option<bool>,
    livemode: Option<bool>,
    name: Option<String>,
    object: Option<RetrievePrescribingOptionsResponseCatalogObject>,
    patient_specific_required: Option<bool>,
    quantity_constraint: Option<RetrievePrescribingOptionsResponseCatalogQuantityConstraint>,
    prescription_requirements:
        Option<RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements>,
    pricing: Option<RetrievePrescribingOptionsResponseCatalogPricing>,
    restricted_states: Option<Vec<String>>,
    route: Option<String>,
    shipping_options: Option<Vec<RetrievePrescribingOptionsResponseCatalogShippingOptionsItem>>,
    strength: Option<String>,
    unit: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogBuilder {
    pub fn catalog_details(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogCatalogDetails,
    ) -> Self {
        self.catalog_details = Some(value);
        self
    }

    pub fn composition(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogComposition,
    ) -> Self {
        self.composition = Some(value);
        self
    }

    pub fn allowed_states(mut self, value: Vec<String>) -> Self {
        self.allowed_states = Some(value);
        self
    }

    pub fn availability(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogAvailability,
    ) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn catalog_kind(mut self, value: impl Into<String>) -> Self {
        self.catalog_kind = Some(value.into());
        self
    }

    pub fn fulfillment_inclusions(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogFulfillmentInclusionsItem>,
    ) -> Self {
        self.fulfillment_inclusions = Some(value);
        self
    }

    pub fn ordering(mut self, value: RetrievePrescribingOptionsResponseCatalogOrdering) -> Self {
        self.ordering = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn cold_ship(mut self, value: bool) -> Self {
        self.cold_ship = Some(value);
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

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn dosage_form(mut self, value: impl Into<String>) -> Self {
        self.dosage_form = Some(value.into());
        self
    }

    pub fn facility_type(mut self, value: impl Into<String>) -> Self {
        self.facility_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn image_urls(mut self, value: Vec<String>) -> Self {
        self.image_urls = Some(value);
        self
    }

    pub fn medication_group(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogMedicationGroup,
    ) -> Self {
        self.medication_group = Some(value);
        self
    }

    pub fn is_orderable(mut self, value: bool) -> Self {
        self.is_orderable = Some(value);
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn object(mut self, value: RetrievePrescribingOptionsResponseCatalogObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn patient_specific_required(mut self, value: bool) -> Self {
        self.patient_specific_required = Some(value);
        self
    }

    pub fn quantity_constraint(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogQuantityConstraint,
    ) -> Self {
        self.quantity_constraint = Some(value);
        self
    }

    pub fn prescription_requirements(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirements,
    ) -> Self {
        self.prescription_requirements = Some(value);
        self
    }

    pub fn pricing(mut self, value: RetrievePrescribingOptionsResponseCatalogPricing) -> Self {
        self.pricing = Some(value);
        self
    }

    pub fn restricted_states(mut self, value: Vec<String>) -> Self {
        self.restricted_states = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn shipping_options(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseCatalogShippingOptionsItem>,
    ) -> Self {
        self.shipping_options = Some(value);
        self
    }

    pub fn strength(mut self, value: impl Into<String>) -> Self {
        self.strength = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalog`].
    /// This method will fail if any of the following fields are not set:
    /// - [`catalog_details`](RetrievePrescribingOptionsResponseCatalogBuilder::catalog_details)
    /// - [`composition`](RetrievePrescribingOptionsResponseCatalogBuilder::composition)
    /// - [`allowed_states`](RetrievePrescribingOptionsResponseCatalogBuilder::allowed_states)
    /// - [`availability`](RetrievePrescribingOptionsResponseCatalogBuilder::availability)
    /// - [`catalog_kind`](RetrievePrescribingOptionsResponseCatalogBuilder::catalog_kind)
    /// - [`fulfillment_inclusions`](RetrievePrescribingOptionsResponseCatalogBuilder::fulfillment_inclusions)
    /// - [`ordering`](RetrievePrescribingOptionsResponseCatalogBuilder::ordering)
    /// - [`cold_ship`](RetrievePrescribingOptionsResponseCatalogBuilder::cold_ship)
    /// - [`pharmacy_id`](RetrievePrescribingOptionsResponseCatalogBuilder::pharmacy_id)
    /// - [`pharmacy_name`](RetrievePrescribingOptionsResponseCatalogBuilder::pharmacy_name)
    /// - [`description`](RetrievePrescribingOptionsResponseCatalogBuilder::description)
    /// - [`dosage_form`](RetrievePrescribingOptionsResponseCatalogBuilder::dosage_form)
    /// - [`facility_type`](RetrievePrescribingOptionsResponseCatalogBuilder::facility_type)
    /// - [`id`](RetrievePrescribingOptionsResponseCatalogBuilder::id)
    /// - [`image_urls`](RetrievePrescribingOptionsResponseCatalogBuilder::image_urls)
    /// - [`is_orderable`](RetrievePrescribingOptionsResponseCatalogBuilder::is_orderable)
    /// - [`livemode`](RetrievePrescribingOptionsResponseCatalogBuilder::livemode)
    /// - [`name`](RetrievePrescribingOptionsResponseCatalogBuilder::name)
    /// - [`object`](RetrievePrescribingOptionsResponseCatalogBuilder::object)
    /// - [`patient_specific_required`](RetrievePrescribingOptionsResponseCatalogBuilder::patient_specific_required)
    /// - [`prescription_requirements`](RetrievePrescribingOptionsResponseCatalogBuilder::prescription_requirements)
    /// - [`restricted_states`](RetrievePrescribingOptionsResponseCatalogBuilder::restricted_states)
    /// - [`route`](RetrievePrescribingOptionsResponseCatalogBuilder::route)
    /// - [`shipping_options`](RetrievePrescribingOptionsResponseCatalogBuilder::shipping_options)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalog, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalog {
            catalog_details: self
                .catalog_details
                .ok_or_else(|| BuildError::missing_field("catalog_details"))?,
            composition: self
                .composition
                .ok_or_else(|| BuildError::missing_field("composition"))?,
            allowed_states: self
                .allowed_states
                .ok_or_else(|| BuildError::missing_field("allowed_states"))?,
            availability: self
                .availability
                .ok_or_else(|| BuildError::missing_field("availability"))?,
            catalog_kind: self
                .catalog_kind
                .ok_or_else(|| BuildError::missing_field("catalog_kind"))?,
            fulfillment_inclusions: self
                .fulfillment_inclusions
                .ok_or_else(|| BuildError::missing_field("fulfillment_inclusions"))?,
            ordering: self
                .ordering
                .ok_or_else(|| BuildError::missing_field("ordering"))?,
            category: self.category,
            cold_ship: self
                .cold_ship
                .ok_or_else(|| BuildError::missing_field("cold_ship"))?,
            pharmacy_id: self
                .pharmacy_id
                .ok_or_else(|| BuildError::missing_field("pharmacy_id"))?,
            pharmacy_name: self
                .pharmacy_name
                .ok_or_else(|| BuildError::missing_field("pharmacy_name"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            dosage_form: self
                .dosage_form
                .ok_or_else(|| BuildError::missing_field("dosage_form"))?,
            facility_type: self
                .facility_type
                .ok_or_else(|| BuildError::missing_field("facility_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            image_url: self.image_url,
            image_urls: self
                .image_urls
                .ok_or_else(|| BuildError::missing_field("image_urls"))?,
            medication_group: self.medication_group,
            is_orderable: self
                .is_orderable
                .ok_or_else(|| BuildError::missing_field("is_orderable"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            patient_specific_required: self
                .patient_specific_required
                .ok_or_else(|| BuildError::missing_field("patient_specific_required"))?,
            quantity_constraint: self.quantity_constraint,
            prescription_requirements: self
                .prescription_requirements
                .ok_or_else(|| BuildError::missing_field("prescription_requirements"))?,
            pricing: self.pricing,
            restricted_states: self
                .restricted_states
                .ok_or_else(|| BuildError::missing_field("restricted_states"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            shipping_options: self
                .shipping_options
                .ok_or_else(|| BuildError::missing_field("shipping_options"))?,
            strength: self.strength,
            unit: self.unit,
        })
    }
}
