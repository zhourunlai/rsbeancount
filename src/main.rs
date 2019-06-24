// This is a part of Rsbeancount.
// Copyright (c) 2019, Zhou.
// See README.md for details.

//! A tool to make `beancount` file through the bill downloaded from payment platform.
//!
//! ```shell
//! cargo run csv/alipay_record_201901.csv bean/2019-01.bean
//! ```

#[macro_use]
extern crate clap;

extern crate chrono;
extern crate csv;
extern crate encoding;

mod enums;
mod processor;
mod transaction;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;

use clap::App;

use encoding::all::GB18030;
use encoding::{DecoderTrap, Encoding};

use enums::{Expenses, Income};
use processor::Processor;
use transaction::Transaction;

fn csv_2_bean(csvpath: &str, beanpath: &str) -> Result<(), Box<Error>> {
    // 导入 csv 文件并处理中文
    let mut csvfile = match File::open(csvpath) {
        Ok(file) => file,
        Err(e) => panic!("couldn't open {}: {}", csvpath, e.description()),
    };
    let mut reader: Vec<u8> = Vec::new();
    csvfile.read_to_end(&mut reader).expect("can not read file");
    let mut chars = String::new();
    let _ = GB18030.decode_to(&reader, DecoderTrap::Ignore, &mut chars);
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
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let csvpath = matches.value_of("csvpath").unwrap();
    let beanpath = matches.value_of("beanpath").unwrap();
    println!("run {} => {}", csvpath, beanpath);

    if let Err(err) = csv_2_bean(csvpath, beanpath) {
        println!("error running: {}", err);
        process::exit(1);
    }
}
