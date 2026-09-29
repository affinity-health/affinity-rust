pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory {
    AlcoholFree,
    DrugShortage,
    CommercialProductDiscontinued,
    ModifiedRelease,
    InactiveIngredientSensitivity,
    InactiveIngredientToxicity,
    ConcentrationAdjustment,
    AlternateRoute,
    DosageFormUnavailable,
    FlavorAdjustment,
    TabletBurden,
    PatientCannotUseCommercialProduct,
    NoApprovedProductAvailable,
    NoRationaleRequired,
    OtherPatientSpecificNeed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AlcoholFree => serializer.serialize_str("alcohol_free"),
            Self::DrugShortage => serializer.serialize_str("drug_shortage"),
            Self::CommercialProductDiscontinued => {
                serializer.serialize_str("commercial_product_discontinued")
            }
            Self::ModifiedRelease => serializer.serialize_str("modified_release"),
            Self::InactiveIngredientSensitivity => {
                serializer.serialize_str("inactive_ingredient_sensitivity")
            }
            Self::InactiveIngredientToxicity => {
                serializer.serialize_str("inactive_ingredient_toxicity")
            }
            Self::ConcentrationAdjustment => serializer.serialize_str("concentration_adjustment"),
            Self::AlternateRoute => serializer.serialize_str("alternate_route"),
            Self::DosageFormUnavailable => serializer.serialize_str("dosage_form_unavailable"),
            Self::FlavorAdjustment => serializer.serialize_str("flavor_adjustment"),
            Self::TabletBurden => serializer.serialize_str("tablet_burden"),
            Self::PatientCannotUseCommercialProduct => {
                serializer.serialize_str("patient_cannot_use_commercial_product")
            }
            Self::NoApprovedProductAvailable => {
                serializer.serialize_str("no_approved_product_available")
            }
            Self::NoRationaleRequired => serializer.serialize_str("no_rationale_required"),
            Self::OtherPatientSpecificNeed => {
                serializer.serialize_str("other_patient_specific_need")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "alcohol_free" => Ok(Self::AlcoholFree),
            "drug_shortage" => Ok(Self::DrugShortage),
            "commercial_product_discontinued" => Ok(Self::CommercialProductDiscontinued),
            "modified_release" => Ok(Self::ModifiedRelease),
            "inactive_ingredient_sensitivity" => Ok(Self::InactiveIngredientSensitivity),
            "inactive_ingredient_toxicity" => Ok(Self::InactiveIngredientToxicity),
            "concentration_adjustment" => Ok(Self::ConcentrationAdjustment),
            "alternate_route" => Ok(Self::AlternateRoute),
            "dosage_form_unavailable" => Ok(Self::DosageFormUnavailable),
            "flavor_adjustment" => Ok(Self::FlavorAdjustment),
            "tablet_burden" => Ok(Self::TabletBurden),
            "patient_cannot_use_commercial_product" => Ok(Self::PatientCannotUseCommercialProduct),
            "no_approved_product_available" => Ok(Self::NoApprovedProductAvailable),
            "no_rationale_required" => Ok(Self::NoRationaleRequired),
            "other_patient_specific_need" => Ok(Self::OtherPatientSpecificNeed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display
    for PreviewOrderRequestPrescriptionsItemOverridesClinicalCompoundingReasonCategory
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlcoholFree => write!(f, "alcohol_free"),
            Self::DrugShortage => write!(f, "drug_shortage"),
            Self::CommercialProductDiscontinued => write!(f, "commercial_product_discontinued"),
            Self::ModifiedRelease => write!(f, "modified_release"),
            Self::InactiveIngredientSensitivity => write!(f, "inactive_ingredient_sensitivity"),
            Self::InactiveIngredientToxicity => write!(f, "inactive_ingredient_toxicity"),
            Self::ConcentrationAdjustment => write!(f, "concentration_adjustment"),
            Self::AlternateRoute => write!(f, "alternate_route"),
            Self::DosageFormUnavailable => write!(f, "dosage_form_unavailable"),
            Self::FlavorAdjustment => write!(f, "flavor_adjustment"),
            Self::TabletBurden => write!(f, "tablet_burden"),
            Self::PatientCannotUseCommercialProduct => {
                write!(f, "patient_cannot_use_commercial_product")
            }
            Self::NoApprovedProductAvailable => write!(f, "no_approved_product_available"),
            Self::NoRationaleRequired => write!(f, "no_rationale_required"),
            Self::OtherPatientSpecificNeed => write!(f, "other_patient_specific_need"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
