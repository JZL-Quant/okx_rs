use crate::dto::common::MarginMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceInfo {
    // 美金层面有效保证金
    pub adj_eq: String,
    // 账户美金层面潜在借币占用保证金
    pub borrow_froz: String,
    // 各币种资产详细信息
    pub details: Vec<Balance>,
    // 美金层面占用保证金
    pub imr: String,
    // 美金层面逐仓仓位权益
    pub iso_eq: String,
    // 美金层面保证金率
    pub mgn_ratio: String,
    // 美金层面维持保证金
    pub mmr: String,
    // 以美金价值为单位的持仓数量，即仓位美金价值
    pub notional_usd: String,
    // 借币金额（美元价值）
    pub notional_usd_for_borrow: String,
    // 交割合约持仓美元价值
    pub notional_usd_for_futures: String,
    // 期权持仓美元价值
    pub notional_usd_for_option: String,
    // 永续合约持仓美元价值
    pub notional_usd_for_swap: String,
    pub ord_froz: String,
    pub total_eq: String,
    pub u_time: String,
    pub upl: String,
}

/// 账户余额信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// 币种
    pub ccy: String,
    /// 币种总额
    #[serde(rename = "cashBal")]
    pub balance: String,
    /// 可用余额
    #[serde(rename = "availBal")]
    pub available_balance: String,
    /// 冻结余额
    #[serde(rename = "frozenBal")]
    pub frozen_balance: String,
    /// 币种负债额
    #[serde(rename = "liab", skip_serializing_if = "Option::is_none")]
    pub liability: Option<String>,
    /// 币种当前可用保证金
    #[serde(rename = "availEq", skip_serializing_if = "Option::is_none")]
    pub available_equity: Option<String>,
    /// 币种风险价值
    #[serde(rename = "upl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<String>,
}

/// 账户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// 账户ID
    #[serde(rename = "acctId")]
    pub account_id: String,
    /// 持仓类型
    #[serde(rename = "posMode")]
    pub position_mode: String,
    /// 是否自动借币
    #[serde(rename = "autoLoan")]
    pub auto_loan: bool,
    /// 账户级别
    pub level: String,
    /// 杠杆模式
    #[serde(rename = "mgnMode")]
    pub margin_mode: MarginMode,
}

/// 账户风险数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRisk {
    /// 当前风险数据
    pub risk: String,
    /// 风险等级
    #[serde(rename = "riskLvl")]
    pub risk_level: String,
    /// 总权益
    #[serde(rename = "totalEq")]
    pub total_equity: String,
}
