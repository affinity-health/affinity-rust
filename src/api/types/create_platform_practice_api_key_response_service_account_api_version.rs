pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreatePlatformPracticeApiKeyResponseServiceAccountApiVersion {
    #[serde(rename = "2026-09-28")]
    TwoThousandTwentySix0928,
}
impl fmt::Display for CreatePlatformPracticeApiKeyResponseServiceAccountApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TwoThousandTwentySix0928 => "2026-09-28",
        };
        write!(f, "{}", s)
    }
}
