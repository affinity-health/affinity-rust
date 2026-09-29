pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListPharmaciesResponseDataItemProfileRating {
    Double(f64),

    ListPharmaciesResponseDataItemProfileRatingOne(ListPharmaciesResponseDataItemProfileRatingOne),
}

impl ListPharmaciesResponseDataItemProfileRating {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_pharmacies_response_data_item_profile_rating_one(&self) -> bool {
        matches!(
            self,
            Self::ListPharmaciesResponseDataItemProfileRatingOne(_)
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

    pub fn as_list_pharmacies_response_data_item_profile_rating_one(
        &self,
    ) -> Option<&ListPharmaciesResponseDataItemProfileRatingOne> {
        match self {
            Self::ListPharmaciesResponseDataItemProfileRatingOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_pharmacies_response_data_item_profile_rating_one(
        self,
    ) -> Option<ListPharmaciesResponseDataItemProfileRatingOne> {
        match self {
            Self::ListPharmaciesResponseDataItemProfileRatingOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ListPharmaciesResponseDataItemProfileRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListPharmaciesResponseDataItemProfileRatingOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
