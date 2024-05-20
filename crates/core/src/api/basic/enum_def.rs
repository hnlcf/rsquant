use serde::{
    Deserialize,
    Serialize,
};

/// # 鉴权类型
///
/// `TRADE` 和 `USER_DATA` 接口是 签名(SIGNED)接口.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum EndpointSecurityType {
    /// 不需要鉴权的接口
    None,
    /// 需要有效的API-KEY和签名
    Trade,
    Margin,
    /// 需要有效的API-KEY和签名
    UserData,
    /// 需要有效的API-KEY
    UserStream,
    /// 需要有效的API-KEY
    MarketData,
}

/// # 交易对状态
///
/// 参数字段名: `status`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum SymbolStatus {
    /// 盘前交易
    PreTrading,
    /// 正常交易中
    Trading,
    /// 盘后交易
    PostTrading,
    /// 收盘
    EndOfDay,
    /// 交易终止(该交易对已下线)
    Halt,
    /// 集合竞价
    AuctionMatch,
    /// 交易暂停
    Break,
}

/// # 账户与交易对权限
///
/// 参数字段名: `permissions`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum AccountAndSymbolPermission {
    /// 现货
    Spot,
    /// 杠杆
    Margin,
    /// 杠杆代币
    Leveraged,
    /// 交易组 002
    TrdGrp002,
    /// 交易组 003
    TrdGrp003,
    /// 交易组 004
    TrdGrp004,
    /// 交易组 005
    TrdGrp005,
    /// 交易组 006
    TrdGrp006,
    /// 交易组 007
    TrdGrp007,
    /// 交易组 008
    TrdGrp008,
    /// 交易组 009
    TrdGrp009,
    /// 交易组 010
    TrdGrp0010,
    /// 交易组 011
    TrdGrp0011,
    /// 交易组 012
    TrdGrp0012,
    /// 交易组 013
    TrdGrp0013,
}

/// # 订单状态
///
/// 参数字段名: `status`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OrderStatus {
    /// 订单被交易引擎接受
    New,
    /// 部分订单被成交
    PartiallyFilled,
    /// 订单完全成交
    Filled,
    /// 用户撤销了订单
    Canceled,
    /// 撤销中(目前并未使用)
    PendingCancel,
    /// 订单没有被交易引擎接受，也没被处理
    Rejected,
    /// 订单被交易引擎取消, 比如 LIMIT FOK 订单没有成交、市价单没有完全成交、强平期间被取消的订单、交易所维护期间被取消的订单
    Expired,
    /// 表示订单由于 STP 而过期 （e.g. 带有 EXPIRE_TAKER 的订单与订单簿上属于同账户或同 tradeGroupId 的订单撮合）
    ExpiredInMatch,
}

/// # OCO 状态
///
/// 参数字段名: `listStatusType`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OcoStatus {
    /// 当ListStatus响应失败的操作时使用。 (订单完成或取消订单)
    Response,
    /// 当已经下单或者订单有更新时
    ExecStarted,
    /// 当订单执行结束或者不在激活状态
    AllDone,
}

/// # OCO 订单状态
///
/// 参数字段名: `listOrderStatus`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OcoOrderStatus {
    /// 当已经下单或者订单有更新时
    Executing,
    /// 当订单执行结束或者不在激活状态
    AllDone,
    /// 当订单状态响应失败(订单完成或取消订单)
    Reject,
}

/// # 指定订单的类型
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum ContingencyType {
    /// 选择性委托订单
    Oco,
}

/// # 订单种类
///
/// 参数字段名: `orderTypes`, `type`.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OrderTypes {
    /// 限价单
    Limit,
    /// 市价单
    Market,
    /// 止损单
    StopLoss,
    /// 限价止损单
    StopLossLimit,
    /// 止盈单
    TakeProfit,
    /// 限价止盈单
    TakeProfitLimit,
    /// 限价做市单
    LimitMaker,
}

/// # 订单返回类型
///
/// 参数字段名: `newOrderRespType`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OrderResponseType {
    Ack,
    Result,
    Full,
}

/// # 订单方向
///
/// 参数字段名: `side`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum OrderSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

/// # 订单失效时间
///
/// 参数字段名: `timeInForce`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum TimeInForce {
    /// 成交为止，订单会一直有效，直到被成交或者取消。
    Gtc,
    /// 无法立即成交的部分就撤销，订单在失效前会尽量多的成交。
    Ioc,
    /// 无法全部立即成交就撤销，如果无法全部成交，订单会失效。
    Fok,
}

/// # K线间隔
///
/// 参数字段名: `interval`
#[derive(Serialize, Deserialize)]
enum KlineChartIntervals {
    #[serde(rename = "1s")]
    OneSeconds,
    #[serde(rename = "1m")]
    OneMinutes,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHours,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "n1d")]
    OneDays,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeeks,
    #[serde(rename = "1M")]
    OneMonths,
}

/// # 限制种类
///
/// 参数字段名: `rateLimitType`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum RateLimiters {
    /// 单位时间请求权重之和上限
    RequestWeight,
    /// 单位时间下单(撤单)次数上限
    Orders,
    /// 单位时间请求次数上限
    RawRequests,
}

/// # 限制间隔
///
/// 参数字段名: `interval`
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum RateLimitIntervals {
    Second,
    Minute,
    Day,
}
