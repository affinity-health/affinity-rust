pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponse {
    #[serde(rename = "compoundingReason")]
    pub compounding_reason: RetrievePrescribingOptionsResponseCompoundingReason,
    #[serde(rename = "compoundingReasonCategoryDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason_category_default:
        Option<RetrievePrescribingOptionsResponseCompoundingReasonCategoryDefault>,
    #[serde(rename = "compoundingReasonDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compounding_reason_default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<RetrievePrescribingOptionsResponseDefault>,
    #[serde(rename = "formulationDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulation_default: Option<RetrievePrescribingOptionsResponseFormulationDefault>,
    #[serde(default)]
    pub initial: RetrievePrescribingOptionsResponseInitial,
    #[serde(default)]
    pub medication: RetrievePrescribingOptionsResponseMedication,
    #[serde(default)]
    pub options: RetrievePrescribingOptionsResponseOptions,
    #[serde(rename = "pharmacyDirections")]
    #[serde(default)]
    pub pharmacy_directions: Vec<RetrievePrescribingOptionsResponsePharmacyDirectionsItem>,
    #[serde(default)]
    pub templates: Vec<RetrievePrescribingOptionsResponseTemplatesItem>,
    pub object: RetrievePrescribingOptionsResponseObject,
    #[serde(rename = "catalogItemId")]
    #[serde(default)]
    pub catalog_item_id: String,
    #[serde(rename = "practiceId")]
    #[serde(default)]
    pub practice_id: String,
    #[serde(default)]
    pub livemode: bool,
    #[serde(default)]
    pub revision: String,
    pub catalog: RetrievePrescribingOptionsResponseCatalog,
    #[serde(rename = "defaultPresetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_preset_id: Option<String>,
    #[serde(default)]
    pub presets: Vec<RetrievePrescribingOptionsResponsePresetsItem>,
}

impl RetrievePrescribingOptionsResponse {
    pub fn builder() -> RetrievePrescribingOptionsResponseBuilder {
        <RetrievePrescribingOptionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseBuilder {
    compounding_reason: Option<RetrievePrescribingOptionsResponseCompoundingReason>,
    compounding_reason_category_default:
        Option<RetrievePrescribingOptionsResponseCompoundingReasonCategoryDefault>,
    compounding_reason_default: Option<String>,
    default: Option<RetrievePrescribingOptionsResponseDefault>,
    formulation_default: Option<RetrievePrescribingOptionsResponseFormulationDefault>,
    initial: Option<RetrievePrescribingOptionsResponseInitial>,
    medication: Option<RetrievePrescribingOptionsResponseMedication>,
    options: Option<RetrievePrescribingOptionsResponseOptions>,
    pharmacy_directions: Option<Vec<RetrievePrescribingOptionsResponsePharmacyDirectionsItem>>,
    templates: Option<Vec<RetrievePrescribingOptionsResponseTemplatesItem>>,
    object: Option<RetrievePrescribingOptionsResponseObject>,
    catalog_item_id: Option<String>,
    practice_id: Option<String>,
    livemode: Option<bool>,
    revision: Option<String>,
    catalog: Option<RetrievePrescribingOptionsResponseCatalog>,
    default_preset_id: Option<String>,
    presets: Option<Vec<RetrievePrescribingOptionsResponsePresetsItem>>,
}

impl RetrievePrescribingOptionsResponseBuilder {
    pub fn compounding_reason(
        mut self,
        value: RetrievePrescribingOptionsResponseCompoundingReason,
    ) -> Self {
        self.compounding_reason = Some(value);
        self
    }

    pub fn compounding_reason_category_default(
        mut self,
        value: RetrievePrescribingOptionsResponseCompoundingReasonCategoryDefault,
    ) -> Self {
        self.compounding_reason_category_default = Some(value);
        self
    }

    pub fn compounding_reason_default(mut self, value: impl Into<String>) -> Self {
        self.compounding_reason_default = Some(value.into());
        self
    }

    pub fn default(mut self, value: RetrievePrescribingOptionsResponseDefault) -> Self {
        self.default = Some(value);
        self
    }

    pub fn formulation_default(
        mut self,
        value: RetrievePrescribingOptionsResponseFormulationDefault,
    ) -> Self {
        self.formulation_default = Some(value);
        self
    }

    pub fn initial(mut self, value: RetrievePrescribingOptionsResponseInitial) -> Self {
        self.initial = Some(value);
        self
    }

    pub fn medication(mut self, value: RetrievePrescribingOptionsResponseMedication) -> Self {
        self.medication = Some(value);
        self
    }

    pub fn options(mut self, value: RetrievePrescribingOptionsResponseOptions) -> Self {
        self.options = Some(value);
        self
    }

    pub fn pharmacy_directions(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponsePharmacyDirectionsItem>,
    ) -> Self {
        self.pharmacy_directions = Some(value);
        self
    }

    pub fn templates(
        mut self,
        value: Vec<RetrievePrescribingOptionsResponseTemplatesItem>,
    ) -> Self {
        self.templates = Some(value);
        self
    }

    pub fn object(mut self, value: RetrievePrescribingOptionsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn catalog_item_id(mut self, value: impl Into<String>) -> Self {
        self.catalog_item_id = Some(value.into());
        self
    }

    pub fn practice_id(mut self, value: impl Into<String>) -> Self {
        self.practice_id = Some(value.into());
        self
    }

    pub fn livemode(mut self, value: bool) -> Self {
        self.livemode = Some(value);
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn catalog(mut self, value: RetrievePrescribingOptionsResponseCatalog) -> Self {
        self.catalog = Some(value);
        self
    }

    pub fn default_preset_id(mut self, value: impl Into<String>) -> Self {
        self.default_preset_id = Some(value.into());
        self
    }

    pub fn presets(mut self, value: Vec<RetrievePrescribingOptionsResponsePresetsItem>) -> Self {
        self.presets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`compounding_reason`](RetrievePrescribingOptionsResponseBuilder::compounding_reason)
    /// - [`initial`](RetrievePrescribingOptionsResponseBuilder::initial)
    /// - [`medication`](RetrievePrescribingOptionsResponseBuilder::medication)
    /// - [`options`](RetrievePrescribingOptionsResponseBuilder::options)
    /// - [`pharmacy_directions`](RetrievePrescribingOptionsResponseBuilder::pharmacy_directions)
    /// - [`templates`](RetrievePrescribingOptionsResponseBuilder::templates)
    /// - [`object`](RetrievePrescribingOptionsResponseBuilder::object)
    /// - [`catalog_item_id`](RetrievePrescribingOptionsResponseBuilder::catalog_item_id)
    /// - [`practice_id`](RetrievePrescribingOptionsResponseBuilder::practice_id)
    /// - [`livemode`](RetrievePrescribingOptionsResponseBuilder::livemode)
    /// - [`revision`](RetrievePrescribingOptionsResponseBuilder::revision)
    /// - [`catalog`](RetrievePrescribingOptionsResponseBuilder::catalog)
    /// - [`presets`](RetrievePrescribingOptionsResponseBuilder::presets)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponse, BuildError> {
        Ok(RetrievePrescribingOptionsResponse {
            compounding_reason: self
                .compounding_reason
                .ok_or_else(|| BuildError::missing_field("compounding_reason"))?,
            compounding_reason_category_default: self.compounding_reason_category_default,
            compounding_reason_default: self.compounding_reason_default,
            default: self.default,
            formulation_default: self.formulation_default,
            initial: self
                .initial
                .ok_or_else(|| BuildError::missing_field("initial"))?,
            medication: self
                .medication
                .ok_or_else(|| BuildError::missing_field("medication"))?,
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
            pharmacy_directions: self
                .pharmacy_directions
                .ok_or_else(|| BuildError::missing_field("pharmacy_directions"))?,
            templates: self
                .templates
                .ok_or_else(|| BuildError::missing_field("templates"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            catalog_item_id: self
                .catalog_item_id
                .ok_or_else(|| BuildError::missing_field("catalog_item_id"))?,
            practice_id: self
                .practice_id
                .ok_or_else(|| BuildError::missing_field("practice_id"))?,
            livemode: self
                .livemode
                .ok_or_else(|| BuildError::missing_field("livemode"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            catalog: self
                .catalog
                .ok_or_else(|| BuildError::missing_field("catalog"))?,
            default_preset_id: self.default_preset_id,
            presets: self
                .presets
                .ok_or_else(|| BuildError::missing_field("presets"))?,
        })
    }
}
