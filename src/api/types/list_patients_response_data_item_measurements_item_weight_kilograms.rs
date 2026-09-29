pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListPatientsResponseDataItemMeasurementsItemWeightKilograms {
    Double(f64),

    ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne(
        ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne,
    ),
}

impl ListPatientsResponseDataItemMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_patients_response_data_item_measurements_item_weight_kilograms_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne(_)
        )
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_list_patients_response_data_item_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_list_patients_response_data_item_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for ListPatientsResponseDataItemMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListPatientsResponseDataItemMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
