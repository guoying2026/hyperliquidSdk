use hyperliquid_sdk::api::HyperliquidApi;
use hyperliquid_sdk::models::{
    Action, Builder, LimitOrder, Order, OrderType, PlaceOrderRequest, TriggerOrder,
};
use tokio;

#[tokio::main]
async fn main() {
    let base_url = "https://api.hyperliquid.xyz".to_string();

    // 创建 HyperliquidApi 实例
    let api = HyperliquidApi::new(base_url);

    // 构造订单
    let order = Order {
        a: 10000,               // 示例资产索引 (现货资产的索引需要 10000 + index)
        b: true,                // 是否为买单
        p: "100.0".to_string(), // 价格
        s: "1.0".to_string(),   // 数量
        r: false,               // 是否为 ReduceOnly
        t: OrderType::Limit {
            limit: LimitOrder {
                tif: "Gtc".to_string(), // 时间策略：GTC, IOC, 或 ALO
            },
        },
        c: None, // 客户端订单 ID（可选）
    };

    // 构造请求体
    let request = PlaceOrderRequest {
        action: Action {
            action_type: "order".to_string(),
            orders: vec![order],          // 订单列表
            grouping: Some("na".to_string()), // 分组策略
            builder: None,                // 构建器信息（可选）
        },
        nonce: chrono::Utc::now().timestamp_millis() as u64, // 使用当前毫秒时间戳
        signature: "dummy_signature".to_string(), // 示例签名，需要替换为实际签名
        vaultAddress: None, // 如果为 vault 下单，填写其地址
    };

    // 调用下单接口
    match api.place_order(request).await {
        Ok(response) => {
            println!("Order placed successfully: {:?}", response);
        }
        Err(e) => {
            eprintln!("Error placing order: {:?}", e);
        }
    }
}