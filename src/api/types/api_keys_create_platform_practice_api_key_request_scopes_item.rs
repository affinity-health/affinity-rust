pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreatePlatformPracticeApiKeyRequestScopesItem {
    CatalogRead,
    SellingPricesRead,
    SellingPricesWrite,
    CatalogPricingRead,
    CatalogPricingWrite,
    FormulationDefaultsRead,
    FormulationDefaultsWrite,
    PracticesRead,
    PracticesWrite,
    ServiceKeysWrite,
    LocationsRead,
    LocationsWrite,
    OrdersRead,
    OrdersWrite,
    OrdersSign,
    PatientsRead,
    PatientsWrite,
    TeamRead,
    TeamWrite,
    HostedSessionsWrite,
    WebhooksRead,
    WebhooksWrite,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreatePlatformPracticeApiKeyRequestScopesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CatalogRead => serializer.serialize_str("catalog:read"),
            Self::SellingPricesRead => serializer.serialize_str("selling_prices:read"),
            Self::SellingPricesWrite => serializer.serialize_str("selling_prices:write"),
            Self::CatalogPricingRead => serializer.serialize_str("catalog_pricing:read"),
            Self::CatalogPricingWrite => serializer.serialize_str("catalog_pricing:write"),
            Self::FormulationDefaultsRead => serializer.serialize_str("formulation_defaults:read"),
            Self::FormulationDefaultsWrite => {
                serializer.serialize_str("formulation_defaults:write")
            }
            Self::PracticesRead => serializer.serialize_str("practices:read"),
            Self::PracticesWrite => serializer.serialize_str("practices:write"),
            Self::ServiceKeysWrite => serializer.serialize_str("service_keys:write"),
            Self::LocationsRead => serializer.serialize_str("locations:read"),
            Self::LocationsWrite => serializer.serialize_str("locations:write"),
            Self::OrdersRead => serializer.serialize_str("orders:read"),
            Self::OrdersWrite => serializer.serialize_str("orders:write"),
            Self::OrdersSign => serializer.serialize_str("orders:sign"),
            Self::PatientsRead => serializer.serialize_str("patients:read"),
            Self::PatientsWrite => serializer.serialize_str("patients:write"),
            Self::TeamRead => serializer.serialize_str("team:read"),
            Self::TeamWrite => serializer.serialize_str("team:write"),
            Self::HostedSessionsWrite => serializer.serialize_str("hosted_sessions:write"),
            Self::WebhooksRead => serializer.serialize_str("webhooks:read"),
            Self::WebhooksWrite => serializer.serialize_str("webhooks:write"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreatePlatformPracticeApiKeyRequestScopesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "catalog:read" => Ok(Self::CatalogRead),
            "selling_prices:read" => Ok(Self::SellingPricesRead),
            "selling_prices:write" => Ok(Self::SellingPricesWrite),
            "catalog_pricing:read" => Ok(Self::CatalogPricingRead),
            "catalog_pricing:write" => Ok(Self::CatalogPricingWrite),
            "formulation_defaults:read" => Ok(Self::FormulationDefaultsRead),
            "formulation_defaults:write" => Ok(Self::FormulationDefaultsWrite),
            "practices:read" => Ok(Self::PracticesRead),
            "practices:write" => Ok(Self::PracticesWrite),
            "service_keys:write" => Ok(Self::ServiceKeysWrite),
            "locations:read" => Ok(Self::LocationsRead),
            "locations:write" => Ok(Self::LocationsWrite),
            "orders:read" => Ok(Self::OrdersRead),
            "orders:write" => Ok(Self::OrdersWrite),
            "orders:sign" => Ok(Self::OrdersSign),
            "patients:read" => Ok(Self::PatientsRead),
            "patients:write" => Ok(Self::PatientsWrite),
            "team:read" => Ok(Self::TeamRead),
            "team:write" => Ok(Self::TeamWrite),
            "hosted_sessions:write" => Ok(Self::HostedSessionsWrite),
            "webhooks:read" => Ok(Self::WebhooksRead),
            "webhooks:write" => Ok(Self::WebhooksWrite),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreatePlatformPracticeApiKeyRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CatalogRead => write!(f, "catalog:read"),
            Self::SellingPricesRead => write!(f, "selling_prices:read"),
            Self::SellingPricesWrite => write!(f, "selling_prices:write"),
            Self::CatalogPricingRead => write!(f, "catalog_pricing:read"),
            Self::CatalogPricingWrite => write!(f, "catalog_pricing:write"),
            Self::FormulationDefaultsRead => write!(f, "formulation_defaults:read"),
            Self::FormulationDefaultsWrite => write!(f, "formulation_defaults:write"),
            Self::PracticesRead => write!(f, "practices:read"),
            Self::PracticesWrite => write!(f, "practices:write"),
            Self::ServiceKeysWrite => write!(f, "service_keys:write"),
            Self::LocationsRead => write!(f, "locations:read"),
            Self::LocationsWrite => write!(f, "locations:write"),
            Self::OrdersRead => write!(f, "orders:read"),
            Self::OrdersWrite => write!(f, "orders:write"),
            Self::OrdersSign => write!(f, "orders:sign"),
            Self::PatientsRead => write!(f, "patients:read"),
            Self::PatientsWrite => write!(f, "patients:write"),
            Self::TeamRead => write!(f, "team:read"),
            Self::TeamWrite => write!(f, "team:write"),
            Self::HostedSessionsWrite => write!(f, "hosted_sessions:write"),
            Self::WebhooksRead => write!(f, "webhooks:read"),
            Self::WebhooksWrite => write!(f, "webhooks:write"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
