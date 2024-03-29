// This is a part of Rsbeancount.
// Copyright (c) 2019, Zhou.
// See README.md for details.

//! Some enums according to the `beancount` syntax.
//! And these are divided into my categories.

use std::fmt;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Source {
    Alipay, // 支付宝
    Wepay,  // 微信支付
    CMB,    // 招商银行
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Income {
    Salary,  // 薪水
    Refer,   // 推荐有奖
    Profit,  // 利息
    Unknown, // 未分类
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expenses {
    Traffic,  // 交通
    Food,     // 吃饭
    Shopping, // 购物
    Unknown,  // 未分类
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Liabilities {
    CreditCardsCMB, // 招商银行信用卡
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Assets {
    Cash,          // 现金
    Stock,         // 股票
    BacnkCardsBOC, // 中国银行储蓄卡
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Equity {
    OpenBalance, // 自补
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Source::Alipay => "Alipay",
            Source::Wepay => "Wepay",
            Source::CMB => "CMB",
        }
    }
}

impl fmt::Display for Income {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match *self {
            Income::Salary => "Income:Salary",
            Income::Refer => "Income:Refer",
            Income::Profit => "Income:Profit",
            Income::Unknown => "Income:Unknown",
        };
        write!(f, "{}", d)
    }
}

impl fmt::Display for Expenses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match *self {
            Expenses::Traffic => "Expenses:Traffic",
            Expenses::Food => "Expenses:Food",
            Expenses::Shopping => "Expenses:Shopping",
            Expenses::Unknown => "Expenses:Unknown",
        };
        write!(f, "{}", d)
    }
}

impl fmt::Display for Liabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match *self {
            Liabilities::CreditCardsCMB => "Liabilities:CreditCards:CMB",
        };
        write!(f, "{}", d)
    }
}

impl fmt::Display for Assets {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match *self {
            Assets::Cash => "Assets:Cash",
            Assets::Stock => "Assets:Stock",
            Assets::BacnkCardsBOC => "Assets:BacnkCards:BOC",
        };
        write!(f, "{}", d)
    }
}

impl fmt::Display for Equity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match *self {
            Equity::OpenBalance => "Equity:OpenBalance",
        };
        write!(f, "{}", d)
    }
}
