use std::fs::File;
use std::io::Write;

use crate::enums::{Expenses, Income};
use crate::transaction::Transaction;

pub trait Processor {
    fn write(&self, beanfile: &mut File, tran: Transaction);
}

impl Processor for Income {
    fn write(&self, beanfile: &mut File, tran: Transaction) {
        write!(
            beanfile,
            "{date} * \"{payee}\" \"{good}\"
        Equity:OpenBalance
        {income}                       {amount} CNY  \n\n",
            date = tran.get_data(),
            payee = tran.payee,
            good = tran.good,
            amount = tran.amount,
            income = self,
        )
        .unwrap()
    }
}

impl Processor for Expenses {
    fn write(&self, beanfile: &mut File, tran: Transaction) {
        write!(
            beanfile,
            "{date} * \"{payee}\" \"{good}\"
        Assets:Cash                       -{amount} CNY
        {expenses}                    {amount} CNY   \n\n",
            date = tran.get_data(),
            payee = tran.payee,
            good = tran.good,
            amount = tran.amount,
            expenses = self,
        )
        .unwrap()
    }
}
