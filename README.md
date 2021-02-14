# ecleaner - eclipse插件清理工具

## 简介

ecleaner是一个eclipse插件清理工具，用于清理eclipse升级后plugins目录下的冗余插件。

## 安装

ecleaner使用rust语言开发，你可以下载源代码自行编译使用：

``` shell
git clone https://github.com/leexgone/ecleaner.git
cd ./ecleaner
cargo build --release
```

如果是Windows用户，可以在[发布版本](https://github.com/leexgone/ecleaner/releases)中下载预编译的`eclean.exe`程序使用。

## 使用

通过eclean命令执行清理。

清理eclipse插件目录并将清理插件备份：

``` shell
eclean c:/eclipse e:/backup/eclipse
```

检测eclipse目录下是否含有可清理的插件：

``` shell
eclean -t c:/eclipse
```

更多使用命令查阅：

``` shell
eclean --help
```

## 更新说明

### V1.0.0

+ 发布第一个版本

### V1.1.0

+ 修改命令参数，在指定-t参数时，可忽略BACKUP参数
