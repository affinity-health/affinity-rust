//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Locations**
//! - **API Keys**
//! - **Account**
//! - **Pharmacies**
//! - **Orders**
//! - **Team**
//! - **Practices**
//! - **Patients**
//! - **Catalog**
//! - **Webhooks**

use crate::{ApiError, ClientConfig};

pub mod account;
pub mod api_keys;
pub mod catalog;
pub mod locations;
pub mod orders;
pub mod patients;
pub mod pharmacies;
pub mod practices;
pub mod team;
pub mod webhooks;
pub struct ApiClient {
    pub config: ClientConfig,
    pub locations: LocationsClient,
    pub api_keys: ApiKeysClient,
    pub account: AccountClient,
    pub pharmacies: PharmaciesClient,
    pub orders: OrdersClient,
    pub team: TeamClient,
    pub practices: PracticesClient,
    pub patients: PatientsClient,
    pub catalog: CatalogClient,
    pub webhooks: WebhooksClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            locations: LocationsClient::new(config.clone())?,
            api_keys: ApiKeysClient::new(config.clone())?,
            account: AccountClient::new(config.clone())?,
            pharmacies: PharmaciesClient::new(config.clone())?,
            orders: OrdersClient::new(config.clone())?,
            team: TeamClient::new(config.clone())?,
            practices: PracticesClient::new(config.clone())?,
            patients: PatientsClient::new(config.clone())?,
            catalog: CatalogClient::new(config.clone())?,
            webhooks: WebhooksClient::new(config.clone())?,
        })
    }
}

pub use account::AccountClient;
pub use api_keys::ApiKeysClient;
pub use catalog::CatalogClient;
pub use locations::LocationsClient;
pub use orders::OrdersClient;
pub use patients::PatientsClient;
pub use pharmacies::PharmaciesClient;
pub use practices::PracticesClient;
pub use team::TeamClient;
pub use webhooks::WebhooksClient;
