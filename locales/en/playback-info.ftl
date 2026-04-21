# Playback info sheet — stream details and labels

# Section headers
source = Source
client-output = Client Output
advanced = Advanced

# Source info row labels
resolution = Resolution
info-dynamic-range = Dynamic Range
video-codec = Video Codec
audio-codec = Audio Codec

# Client output row labels
video-delivery = Video Delivery
audio-delivery = Audio Delivery
decoding = Decoding
zero-copy = Zero Copy

# Technical values
hardware = Hardware
software = Software
na = N/A
uploaded = Uploaded
sdr = SDR
hdr = HDR
dolby-vision = Dolby Vision

# Tab labels
tab-video = Video
tab-audio = Audio
tab-subtitles = Subtitles

# Placeholders
no-video-stream = No video stream
no-audio-streams = No audio streams
no-subtitles = No subtitles

# Advanced stream info labels
container = Container
title = Title
overall-bitrate = Overall Bitrate
file-size = File Size
streams = Streams
start-offset = Start Offset
stream = Stream
codec = Codec
profile = Profile
bitrate = Bitrate
bit-depth = Bit Depth
sample-aspect-ratio = Sample Aspect Ratio
color-space = Color Space
color-transfer = Color Transfer
color-primaries = Color Primaries
color-range = Color Range
mastering-display = Mastering Display
primaries-r = Primaries R
primaries-g = Primaries G
primaries-b = Primaries B
white-point = White Point
luminance = Luminance
hdr10plus-metadata = HDR10+ Metadata
present = Present
dolby-vision-profile = Dolby Vision Profile
dolby-vision-level = Dolby Vision Level
dolby-vision-layers = Dolby Vision Layers
bl-compatibility = BL Compatibility
matroska-stereo-mode = Matroska Stereo Mode
stereo-3d = Stereo 3D
mv-hevc-views = MV-HEVC Views
language = Language
channels = Channels
channel-layout = Channel Layout
sample-rate = Sample Rate
forced = Forced
sdh = SDH
selected-suffix = (Selected)

# Stream header format
stream-header-suffix = — { $codec_info }

# Color space values
cs-bt709 = BT.709
cs-bt601 = BT.601
cs-bt2020-ncl = BT.2020 NCL
cs-ipt-pq-c2 = IPT-PQ-c2
cs-unknown = Unknown

# Color transfer values
ct-bt709 = BT.709
ct-pq = PQ (SMPTE 2084)
ct-hlg = HLG (ARIB STD-B67)
ct-unknown = Unknown

# Color primaries values
cp-bt709 = BT.709
cp-bt470-bg = BT.470 BG (PAL)
cp-smpte170m = SMPTE 170M (NTSC)
cp-bt2020 = BT.2020
cp-unknown = Unknown

# Color range values
cr-limited = Limited (TV)
cr-full = Full (PC)
cr-unknown = Unknown

# Dolby Vision profile descriptions
dv-hlg-cross-compat = HLG cross-compat
dv-ipt-pq-no-compat = IPT-PQ-c2 (no compat)
dv-uhd-bluray = UHD Blu-ray (BL+EL+RPU)
dv-hdr10-cross-compat = HDR10 cross-compat (8.1)
dv-sdr-cross-compat = SDR cross-compat (8.2)
dv-hlg-cross-compat-84 = HLG cross-compat (8.4)
dv-single-layer = single-layer
dv-av1 = AV1 DV

# Dolby Vision layer labels
dv-layer-bl = BL
dv-layer-el = EL
dv-layer-rpu = RPU

# Dolby Vision compatibility
dv-compat-none = Dolby Vision (no compat)
dv-compat-hdr10 = HDR10
dv-compat-sdr = SDR (BT.709)
dv-compat-hlg = HLG
dv-compat-bt2020-sdr = BT.2020 SDR
dv-compat-unknown = Unknown ({ $id })

# Stereo 3D types
stereo-2d = 2D
stereo-side-by-side = Side-by-Side
stereo-top-bottom = Top-Bottom
stereo-frame-sequence = Frame Sequence
stereo-checkerboard = Checkerboard
stereo-side-by-side-quincunx = Side-by-Side Quincunx
stereo-lines = Lines
stereo-columns = Columns
stereo-unknown = Unknown

# Channel count labels
channels-mono = Mono
channels-stereo = Stereo
channels-51 = 5.1
channels-71 = 7.1
channels-n = { $count }ch

# Format strings
format-mbps = { $value } Mbps
format-kbps = { $value } kbps
format-bps = { $value } bps
format-gb = { $value } GB
format-mb = { $value } MB
format-kb = { $value } KB
format-bytes = { $value } B
format-hz = { $value } Hz
format-bit-depth = { $value }-bit
format-sar = { $w }:{ $h }
format-offset = { $value }s
format-cd-m2 = { $value } cd/m²
format-luminance-range = { $min }–{ $max } cd/m²
format-mastering-primary = { $x }, { $y }
format-dash = —

# Track fallback
track-fallback = Track { $index }
unknown-language = Unknown
