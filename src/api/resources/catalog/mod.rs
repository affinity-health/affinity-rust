use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod items;
pub use items::ItemsClient;
pub mod shipping_options;
pub use shipping_options::ShippingOptionsClient;
pub mod prescribing_options;
pub use prescribing_options::PrescribingOptionsClient;
pub mod selling_prices;
pub use selling_prices::SellingPricesClient;
pub mod presentation_prices;
pub use presentation_prices::PresentationPricesClient;
pub struct CatalogClient {
    pub http_client: HttpClient,
    pub items: ItemsClient,
    pub shipping_options: ShippingOptionsClient,
    pub prescribing_options: PrescribingOptionsClient,
    pub selling_prices: SellingPricesClient,
    pub presentation_prices: PresentationPricesClient,
}

impl CatalogClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            items: ItemsClient::new(config.clone())?,
            shipping_options: ShippingOptionsClient::new(config.clone())?,
            prescribing_options: PrescribingOptionsClient::new(config.clone())?,
            selling_prices: SellingPricesClient::new(config.clone())?,
            presentation_prices: PresentationPricesClient::new(config.clone())?,
        })
    }
}
