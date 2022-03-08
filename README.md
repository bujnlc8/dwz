# 缩短网址 [![Build](https://github.com/bujnlc8/dwz/actions/workflows/dwz.yaml/badge.svg)](https://github.com/bujnlc8/dwz/actions/workflows/dwz.yaml)


## 缘由

学习了一段时间的Rust，始终有种云里雾里的感觉，正好最近工作中有缩短网址的需求，别家的服务要不不稳定，要不收费，于是用Rust写了这个小玩意。


## 实现

web框架采用的是最近比较火的`actix-web`，orm用的是`diesel`，生成短链接的方式也很简单，对原始链接采用`murmurhash3`算法，截取前面的6位字符，如果有冲突再尝试截取倒数6位，再冲突就报错了。

## 部署

采用`github actions` + `docker` + `ansible` 自动化部署。


**⚠️ 本项目仅供学习交流，切勿非法使用**
