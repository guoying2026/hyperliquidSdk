use reqwest::Client;
use crate::models::{PlaceOrderRequest, PlaceOrderResponse};
use std::error::Error;

pub struct HyperliquidApi;

impl HyperliquidApi {
    /// 下单功能 - 静态方法
    pub async fn place_order(
        client: &Client,
        base_url: &str,
        order: PlaceOrderRequest,
    ) -> Result<PlaceOrderResponse, Box<dyn Error>> {
        let url = format!("{}/exchange", base_url);

        // 调试打印请求内容
        println!("Request URL: {}", url);
        println!("Request Body: {:?}", serde_json::to_string(&order).unwrap());

        // 发送请求
        let response = client.post(&url).json(&order).send().await;

        match response {
            Ok(resp) => {
                let status = resp.status();

                if !status.is_success() {
                    let error_body = resp.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                    println!("Error Response Status: {}", status);
                    println!("Error Response Body: {}", error_body);

                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Request failed with status: {}", status),
                    )));
                }

                // 尝试解析 JSON 响应
                let parsed_response = resp.json::<PlaceOrderResponse>().await;
                match parsed_response {
                    Ok(data) => {
                        println!("Parsed Response: {:?}", data);
                        Ok(data)
                    }
                    Err(e) => {
                        println!("JSON parsing error: {:?}", e);
                        Err(Box::new(e))
                    }
                }
            }
            Err(e) => {
                println!("Request error: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
}