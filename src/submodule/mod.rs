// lib.rs
mod models;
mod api;

use models::{PlaceOrderRequest, PlaceOrderResponse};
use api::HyperliquidApi;

pub struct HyperliquidSDK {
    api: HyperliquidApi,
}

impl HyperliquidSDK {
    pub fn new(api: HyperliquidApi) -> Self {
        HyperliquidSDK { api }
    }

    // 通过 Hyperliquid API 下单
    pub async fn place_order(&self, order: PlaceOrderRequest) -> Result<PlaceOrderResponse, reqwest::Error> {
        self.api.place_order(order).await
    }
}