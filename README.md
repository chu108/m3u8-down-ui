
![m3u8-down-ui-Build](https://img.shields.io/github/workflow/status/heisir2014/M3U8-Downloader/M3U8-Downloader-Build?style=flat-square)

# m3u8-down-ui [ 可视化下载工具 ]
m3u8-down-ui是基于 Tauri + React 框架开发的一款可以下载、HLS视频流的APP，功能特点如下：

| 功能 | 支持 |
| :-- | --: |
| 本地M3U8文件下载 | ✓ |
| M3U8 直播源下载 | ✓ |
| 代理设置 | ✓ |
| 多线程下载，指定线程数 | ✓ |

<div align="center">
    <br>
    <img width="739" src="https://github.com/chu108/m3u8-down-ui/blob/master/public/m3ui.gif" alt="M3U8-Downloader">
    <br>
</div>

# 功能规划

目前只做了多线程下载功能，同一时间只能下载一个任务，后续会更新：
| 功能列表 |
| :-- | 
| 暂时、恢复下载  |
| 多任务下载 |
| 自定义Http协议头下载 |
| 自定义KEY和IV解密 |

# 开发环境
#### NodeJS + Rust + Tauri + React

# 依赖工具
#### FFmpeg 需要预先安装

# 参考项目
部分代码，使用了 [VSD](https://github.com/clitic/vsd) 包，去掉了不需要功能并升级了所有依赖版本



