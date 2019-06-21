extern crate chrono;
extern crate csv;
extern crate encoding;

use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use chrono::NaiveDateTime;

#[derive(Debug)]
struct Transaction {
    trade_no: String,    // 交易号
    order_no: String,    // 商家订单号
    created_at: String,  // 交易创建时间
    pay_at: String,      // 付款时间
    updated_at: String,  // 最近修改时间
    come_from: String,   // 交易来源地
    source: String,      // 类型
    payee: String,       // 交易对方
    good: String,        // 商品名称
    amount: String,      // 金额（元）
    drcr: String,        // 收/支
    pay_status: String,  // 交易状态
    service_fee: String, // 服务费（元）
    refund: String,      // 成功退款（元）
    remark: String,      // 备注
    fund_status: String, // 资金状态
}

impl Transaction {
    fn new(record: csv::StringRecord) -> Self {
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
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime = NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S").expect("");
        write!(
            f,
            "{date} * \"{payee}\" \"{good}\"
    Assets:Cash                    -{amount} CNY
    Expenses:Traffic                {amount} CNY
",
            date = datetime.date(),
            payee = self.payee,
            good = self.good,
            amount = self.amount
        )
    }
}

fn bean() -> Result<(), Box<Error>> {
    let args: Vec<String> = env::args().collect();
    let csvpath = &args[1];
    let beanpath = &args[2];

    // 导入 csv 文件并处理中文
    let mut csvfile = match File::open(csvpath) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", csvpath, e.description()),
    };
    let mut reader: Vec<u8> = Vec::new();
    csvfile.read_to_end(&mut reader).ok().expect("can not read file");
    let mut chars = String::new();
    let _ = GB18030.decode_to(&mut reader, DecoderTrap::Ignore, &mut chars);
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(chars.as_bytes());

    // 创建 bean 文件
    let mut beanfile = match File::create(beanpath) {
        Ok(file) => file,
        Err(e) => panic!("couldn't create {}: {}", beanpath, e.description()),
    };

    for (i, result) in rdr.records().enumerate() {
        let record = result?;
        // 表格过滤
        if i < 5 || record.len() != 17 {
            continue;
        }

        let tran = Transaction::new(record);
        let date = NaiveDateTime::parse_from_str(&tran.created_at, "%Y-%m-%d %H:%M:%S")
            .expect(&tran.created_at)
            .date();

        // 规则
        // you can add more
        if tran.drcr == "收入" {
            let mut income = "Unknown";
            if tran.payee == "支付宝推荐赏金" {
                income = "Refer";
            }

            write!(
                &mut beanfile,
                "{date} * \"{payee}\" \"{good}\"
    Equity:OpenBalance
    Income:{income}                    {amount} CNY \n\n",
                date = date,
                payee = tran.payee,
                good = tran.good,
                amount = tran.amount,
                income = income
            )?;
        }

        if tran.drcr == "支出" {
            let mut expenses = "Unknown";
            if tran.payee == "上海公共交通卡股份有限公司" {
                expenses = "Traffic";
            }

            write!(
                &mut beanfile,
                "{date} * \"{payee}\" \"{good}\"
    Assets:Cash                       -{amount} CNY
    Expenses:{expenses}                {amount} CNY \n\n",
                date = date,
                payee = tran.payee,
                good = tran.good,
                amount = tran.amount,
                expenses = expenses
            )?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = bean() {
        println!("error running bean: {}", err);
        process::exit(1);
    }
}
