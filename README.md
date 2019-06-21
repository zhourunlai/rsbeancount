# Rsbeancount

A tool to make `beancount` file through the bill downloaded from payment platform.

## TODO

- [X] [Alipay](https://consumeprod.alipay.com/record/advanced.htm)
- [ ] Wechat Pay
- [ ] China Merchants Bank(salary card)

### Usage

Convert

```shell
cargo run csv/alipay_record_201901.csv bean/2019-01.bean
```

View

```shell
fava bean/main.bean
```

<img src="https://github.com/zhourunlai/rsbeancount/raw/master/bean.png">

<img src="https://github.com/zhourunlai/rsbeancount/raw/master/fava.png">

## Reference

1. [Beancount —— 命令行复式簿记](https://wzyboy.im/post/1063.html)
