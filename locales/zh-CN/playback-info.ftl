# Playback info sheet — stream details and labels

# Section headers
source = 来源
client-output = 客户端输出
advanced = 高级

# Source info row labels
resolution = 分辨率
info-dynamic-range = 动态范围
video-codec = 视频编码
audio-codec = 音频编码

# Client output row labels
video-delivery = 视频传输
audio-delivery = 音频传输
decoding = 解码
zero-copy = 零拷贝

# Technical values
hardware = 硬件
software = 软件
na = 不适用
uploaded = 已上传
sdr = SDR
hdr = HDR
dolby-vision = Dolby Vision

# Tab labels
tab-video = 视频
tab-audio = 音频
tab-subtitles = 字幕

# Placeholders
no-video-stream = 无视频流
no-audio-streams = 无音频流
no-subtitles = 无字幕

# Advanced stream info labels
container = 容器
title = 标题
overall-bitrate = 总码率
file-size = 文件大小
streams = 流
start-offset = 起始偏移
stream = 流
codec = 编码
profile = 配置
bitrate = 码率
bit-depth = 位深
sample-aspect-ratio = 样本宽高比
color-space = 色彩空间
color-transfer = 色彩传输
color-primaries = 色彩原色
color-range = 色彩范围
mastering-display = 母版显示
primaries-r = 原色 R
primaries-g = 原色 G
primaries-b = 原色 B
white-point = 白点
luminance = 亮度
hdr10plus-metadata = HDR10+ 元数据
present = 存在
dolby-vision-profile = Dolby Vision 配置
dolby-vision-level = Dolby Vision 级别
dolby-vision-layers = Dolby Vision 图层
bl-compatibility = BL 兼容性
matroska-stereo-mode = Matroska 立体模式
stereo-3d = 立体 3D
mv-hevc-views = MV-HEVC 视图
language = 语言
channels = 声道
channel-layout = 声道布局
sample-rate = 采样率
forced = 强制
sdh = SDH
selected-suffix = (已选择)

# Stream header format
stream-header-suffix = — { $codec_info }

# Color space values
cs-bt709 = BT.709
cs-bt601 = BT.601
cs-bt2020-ncl = BT.2020 NCL
cs-ipt-pq-c2 = IPT-PQ-c2
cs-unknown = 未知

# Color transfer values
ct-bt709 = BT.709
ct-pq = PQ (SMPTE 2084)
ct-hlg = HLG (ARIB STD-B67)
ct-unknown = 未知

# Color primaries values
cp-bt709 = BT.709
cp-bt470-bg = BT.470 BG (PAL)
cp-smpte170m = SMPTE 170M (NTSC)
cp-bt2020 = BT.2020
cp-unknown = 未知

# Color range values
cr-limited = 有限 (TV)
cr-full = 完整 (PC)
cr-unknown = 未知

# Dolby Vision profile descriptions
dv-hlg-cross-compat = HLG 交叉兼容
dv-ipt-pq-no-compat = IPT-PQ-c2 (无兼容)
dv-uhd-bluray = UHD 蓝光 (BL+EL+RPU)
dv-hdr10-cross-compat = HDR10 交叉兼容 (8.1)
dv-sdr-cross-compat = SDR 交叉兼容 (8.2)
dv-hlg-cross-compat-84 = HLG 交叉兼容 (8.4)
dv-single-layer = 单图层
dv-av1 = AV1 DV

# Dolby Vision layer labels
dv-layer-bl = BL
dv-layer-el = EL
dv-layer-rpu = RPU

# Dolby Vision compatibility
dv-compat-none = Dolby Vision (无兼容)
dv-compat-hdr10 = HDR10
dv-compat-sdr = SDR (BT.709)
dv-compat-hlg = HLG
dv-compat-bt2020-sdr = BT.2020 SDR
dv-compat-unknown = 未知 ({ $id })

# Stereo 3D types
stereo-2d = 2D
stereo-side-by-side = 左右并排
stereo-top-bottom = 上下排列
stereo-frame-sequence = 帧序列
stereo-checkerboard = 棋盘格
stereo-side-by-side-quincunx = 左右交错
stereo-lines = 行交错
stereo-columns = 列交错
stereo-unknown = 未知

# Channel count labels
channels-mono = 单声道
channels-stereo = 立体声
channels-51 = 5.1
channels-71 = 7.1
channels-n = { $count }声道

# Format strings
format-mbps = { $value } Mbps
format-kbps = { $value } kbps
format-bps = { $value } bps
format-gb = { $value } GB
format-mb = { $value } MB
format-kb = { $value } KB
format-bytes = { $value } B
format-hz = { $value } Hz
format-bit-depth = { $value } 位
format-sar = { $w }:{ $h }
format-offset = { $value }秒
format-cd-m2 = { $value } cd/m²
format-luminance-range = { $min }–{ $max } cd/m²
format-mastering-primary = { $x }, { $y }
format-dash = —

# Track fallback
track-fallback = 轨道 { $index }
unknown-language = 未知
