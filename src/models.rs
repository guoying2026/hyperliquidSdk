use serde::{Deserialize, Serialize};

/// 表示限价订单的具体属性
#[derive(Debug, Serialize)]
pub struct LimitOrder {
    /// 有效时间：ALO (仅添加流动性)、IOC (立即成交或取消)、GTC (有效直到取消)
    pub tif: String,
}

/// 表示触发订单的具体属性
#[derive(Debug, Serialize)]
pub struct TriggerOrder {
    /// 是否为市价单
    pub isMarket: bool,
    /// 触发价格
    pub triggerPx: String,
    /// 止盈或止损标志 (可选值：tp 或 sl)
    pub tpsl: Option<String>,
}

/// 订单类型的枚举类型，可以是限价订单或触发订单
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum OrderType {
    /// 限价订单
    Limit { limit: LimitOrder },
    /// 触发订单
    Trigger { trigger: TriggerOrder },
}

/// 表示一个具体的订单
#[derive(Debug, Serialize)]
pub struct Order {
    /// 资产的索引编号
    /// 永续合约的资产索引来源于 meta 响应的 universe 字段
    /// 现货资产的索引需要使用 10000 + index (index 来源于 spotMeta.universe)
    pub a: u32,
    /// 是否为买单 (true 表示买单，false 表示卖单)
    pub b: bool,
    /// 订单价格
    pub p: String,
    /// 订单数量
    pub s: String,
    /// 是否为 reduce-only（仅减少头寸）
    pub r: bool,
    /// 订单类型 (限价订单或触发订单)
    pub t: OrderType,
    /// 客户订单 ID（可选字段），128 位十六进制字符串
    pub c: Option<String>,
}

/// 下单请求的主体
#[derive(Debug, Serialize)]
pub struct PlaceOrderRequest {
    /// 动作描述
    pub action: Action,
    /// 防重放随机数，推荐使用当前毫秒时间戳
    pub nonce: u64,
    /// 签名信息，需根据请求体生成
    pub signature: String,
    /// 如果代表 vault 进行交易，需要提供链上的 vault 地址
    pub vaultAddress: Option<String>,
}

/// 表示动作内容，包含订单及分组信息
#[derive(Debug, Serialize)]
pub struct Action {
    /// 动作类型，始终为 "order"
    #[serde(rename = "type")]
    pub action_type: String,
    /// 包含的订单列表
    pub orders: Vec<Order>,
    /// 分组策略 (可选值：na、normalTpsl、positionTpsl)
    pub grouping: Option<String>,
    /// 构建器信息 (可选字段)
    pub builder: Option<Builder>,
}

/// 构建器信息（用于额外费用的配置）
#[derive(Debug, Serialize)]
pub struct Builder {
    /// 接收额外费用的地址
    pub b: String,
    /// 费用大小，以基点的十分之一为单位
    /// 例如：如果 f 为 10，则费用为订单名义金额的 1bp
    pub f: u32,
}

/// 下单响应的数据结构
#[derive(Debug, Deserialize)]
pub struct PlaceOrderResponse {
    /// 响应状态，"ok" 表示成功
    pub status: String,
    /// 响应数据，包含订单状态等信息
    pub response: Option<ResponseData>,
}

/// 响应数据结构，包含订单的详细状态
#[derive(Debug, Deserialize)]
pub struct ResponseData {
    /// 响应类型，始终为 "order"
    #[serde(rename = "type")]
    pub response_type: String,
    /// 数据字段，包含订单状态信息
    pub data: Option<ResponseStatuses>,
}

/// 订单状态的集合
#[derive(Debug, Deserialize)]
pub struct ResponseStatuses {
    /// 包含多个订单状态
    pub statuses: Vec<ResponseStatus>,
}

/// 单个订单的状态，可以是 resting 状态或错误信息
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseStatus {
    /// 订单 resting 状态，包含订单 ID
    Resting { resting: RestingData },
    /// 错误状态，包含错误信息
    Error { error: String },
}

/// 订单的 resting 数据，包含订单 ID
#[derive(Debug, Deserialize)]
pub struct RestingData {
    /// 订单 ID
    pub oid: u64,
}