pub mod models;
pub mod api;

use models::{PlaceOrderRequest, PlaceOrderResponse};
pub use api::HyperliquidApi;

pub struct HyperliquidSDK {
    api: HyperliquidApi,
}

impl HyperliquidSDK {
    pub fn new(api: HyperliquidApi) -> Self {
        HyperliquidSDK { api }
    }

    // 通过 Hyper liquid API 下单
    pub async fn place_order(&self, order: PlaceOrderRequest) -> Result<PlaceOrderResponse, reqwest::Error> {
        self.api.place_order(order).await
    }
}