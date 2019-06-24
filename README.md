# Rsbeancount

[![Build Status](https://travis-ci.org/zhourunlai/rsbeancount.svg?branch=master)](https://travis-ci.org/zhourunlai/rsbeancount)
[![GitHub license](https://img.shields.io/github/license/zhourunlai/rsbeancount.svg)](https://raw.githubusercontent.com/zhourunlai/rsbeancount/master/LICENSE)

[![buymeacoffee](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/xiaorun)

A tool to make `beancount` file through the bill downloaded from payment platform.

## TODO

- [X] [Alipay](https://consumeprod.alipay.com/record/advanced.htm)
- [ ] Wechat Pay
- [ ] China Merchants Bank(credit card)

### Usage

1. `cargo build`
2. `./target/debug/rsbeancount -c <csvpath> -b <beanpath>`
3. more: `./target/debug/rsbeancount --help`

<img src="https://github.com/zhourunlai/rsbeancount/raw/master/bean.png">

### View

1. `pip3 install beancount`
2. `pip3 install fava`
3. `fava bean/main.bean`

<img src="https://github.com/zhourunlai/rsbeancount/raw/master/fava.png">

## Reference

1. [Beancount —— 命令行复式簿记](https://wzyboy.im/post/1063.html)
