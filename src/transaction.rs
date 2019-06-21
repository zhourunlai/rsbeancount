use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug)]
pub struct Transaction {
    pub trade_no: String,    // 交易号
    pub order_no: String,    // 商家订单号
    pub created_at: String,  // 交易创建时间
    pub pay_at: String,      // 付款时间
    pub updated_at: String,  // 最近修改时间
    pub come_from: String,   // 交易来源地
    pub source: String,      // 类型
    pub payee: String,       // 交易对方
    pub good: String,        // 商品名称
    pub amount: String,      // 金额（元）
    pub drcr: String,        // 收/支
    pub pay_status: String,  // 交易状态
    pub service_fee: String, // 服务费（元）
    pub refund: String,      // 成功退款（元）
    pub remark: String,      // 备注
    pub fund_status: String, // 资金状态
}

impl Transaction {
    pub fn new(record: csv::StringRecord) -> Self {
        Transaction {
            trade_no: record.get(0).unwrap_or("").trim().to_string(),
            order_no: record.get(1).unwrap_or("").trim().to_string(),
            created_at: record.get(2).unwrap_or("").trim().to_string(),
            pay_at: record.get(3).unwrap_or("").trim().to_string(),
            updated_at: record.get(4).unwrap_or("").trim().to_string(),
            come_from: record.get(5).unwrap_or("").trim().to_string(),
            source: record.get(6).unwrap_or("").trim().to_string(),
            payee: record.get(7).unwrap_or("").trim().to_string(),
            good: record.get(8).unwrap_or("").trim().to_string(),
            amount: record.get(9).unwrap_or("").trim().to_string(),
            drcr: record.get(10).unwrap_or("").trim().to_string(),
            pay_status: record.get(11).unwrap_or("").trim().to_string(),
            service_fee: record.get(2).unwrap_or("").trim().to_string(),
            refund: record.get(13).unwrap_or("").trim().to_string(),
            remark: record.get(14).unwrap_or("").trim().to_string(),
            fund_status: record.get(15).unwrap_or("").trim().to_string(),
        }
    }

    pub fn get_data(&self) -> NaiveDate {
        NaiveDateTime::parse_from_str(self.created_at.as_ref(), "%Y-%m-%d %H:%M:%S")
            .expect(self.created_at.as_ref())
            .date()
    }
}
