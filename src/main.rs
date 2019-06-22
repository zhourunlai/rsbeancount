extern crate chrono;
extern crate csv;
extern crate encoding;

mod enums;
mod processor;
mod transaction;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use enums::{Expenses, Income};
use processor::Processor;
use transaction::Transaction;

fn csv_2_bean() -> Result<(), Box<Error>> {
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

        // 规则
        if tran.drcr == "收入" {
            let income = match tran.payee.as_ref() {
                "支付宝推荐赏金" => Income::Refer,
                "博时基金管理有限公司" => Income::Profit,
                _ => Income::Unknown,
            };
            income.write(&mut beanfile, tran);
        } else if tran.drcr == "支出" {
            let expenses = match tran.payee.as_ref() {
                "上海公共交通卡股份有限公司" | "上海都畅数字技术有限公司" => Expenses::Traffic,
                "饿了么" | "美团点评" => Expenses::Food,
                _ => Expenses::Unknown,
            };
            expenses.write(&mut beanfile, tran);
        }
    }
    Ok(())
}

pub fn main() {
    if let Err(err) = csv_2_bean() {
        println!("error running bean: {}", err);
        process::exit(1);
    }
}
