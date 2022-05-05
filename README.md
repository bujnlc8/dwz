# 缩短网址 [![Build](https://github.com/bujnlc8/dwz/actions/workflows/dwz.yaml/badge.svg)](https://github.com/bujnlc8/dwz/actions/workflows/dwz.yaml)


## 缘由

学习了一段时间的Rust，始终有种云里雾里的感觉，正好最近工作中有缩短网址的需求，别家的服务要不不稳定，要不收费，于是用Rust写了这个小玩意。


## 实现

web框架采用的是最近比较火的`actix-web`，orm用的是`diesel`，生成短链接的方式也很简单，对原始链接采用`murmurhash3`算法，截取前面的6位字符，如果有冲突再尝试截取倒数6位，再冲突就报错了。

## 部署

1.采用`github actions` + `docker` + `ansible` 自动化部署，这种方式通过打格式为`release-*.*.*`的tag触发github的action实现，详见[https://github.com/bujnlc8/dwz/blob/master/.github/workflows/dwz.yaml](https://github.com/bujnlc8/dwz/blob/master/.github/workflows/dwz.yaml).

2.直接拉取打包好的docker镜像手动部署，命令如下：

```sh
docker pull yy194131/dwz:latest

docker run --restart=always -e DATABASE_URL=mysql://user:password@host/db -e DWZ_PORT=8080 -e DWZ_HOST=dwz0.tk --name=dwz -p 8080:8080 -d yy194131/dwz:latest

```

用到的环境变量如下:

|环境变量|例子|备注|
| :----- | :--- | :--- |
| DATABASE_URL | mysql://user:password@127.0.0.1/dwz | 保存数据的mysql dsn，sql见`https://github.com/bujnlc8/dwz/blob/master/migrations/2022-03-07-083823_dwz/up.sql` |
| DWZ_PORT | 8080 | web服务监听的端口，不传默认是8080 |
| DWZ_HOST | dwz0.tk | 短网址域名，不传默认dwz0.tk |


如果采用自动化部署，需将环境变量加到github的Secrets中。


**⚠️ 本项目仅供学习交流，切勿非法使用**
