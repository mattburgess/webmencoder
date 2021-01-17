# webmencoder

## Introduction

`webmencoder` is an opiniated `ffmpeg` wrapper tool that converts MPEG files to WebM format.

## Reason For Existence

FFmpeg is an incredibly useful tool for converting source media into other formats that are more space-efficient. As well as there being a large number of choices to make regarding the output media format, target quality/bit-rates, etc. target-device compatibility can further constrain what choices can be made. The end result of this is that the FFmpeg command line can become quite a complex beast, and having to construct that on a source by source basis can be both frustrating and error prone.

This tool automates the process of invoking FFmpeg with the correct command line options based on the input media and the opinions described below.

## Webmencoder's Opinions

### Media File Format

* Container: WebM - it's open and royalty-free. As such, it's supported by lots of open source media players and web browsers

### Video Encoding

* Codec: VP9 - it's open and royalty-free. It also has good support in open source media players and web browsers. Whilst AV1 is [recommended](https://developer.mozilla.org/en-US/docs/Web/Media/Formats/Video_codecs#Recommendations_for_high-quality_video_presentation), it lacks [hardware encoding support](https://en.wikipedia.org/wiki/Intel_Quick_Sync_Video), at least on recent Intel CPUs, and software encoding is still pretty slow in comparison to VP9.
* Encoding method: Hardware - best current practice suggests the use of 2-pass encoding to obtain reasonable video quality using software encoding. On my hardware, it takes on average 25% of the video duration to complete the 1st pass, and 250% to complete the 2nd pass, for a total duration of 275%. In contrast, hardware encoding completes in 16% of the video duration and requires only a single pass to achieve similar quality levels. Using software, it will take approximately 124 minutes to encode a 45 minute video compared to just 7 minutes using hardware.
* Resolution, frame-rate, etc.: Original media resolutions will be used for the encoded video. This leaves the client device in control of any up- or down-scaling required.

### Audio Encoding

* Codec: Vorbis - Although the WebM container format supports both Vorbis and Opus codecs, some of my existing devices [don't support Opus natively](https://developer.samsung.com/smarttv/develop/specifications/media-specifications/2018-tv-video-specifications.html), resulting in my NAS having to transcode media on the fly. If it wasn't for that issue, I'd probably have selected Opus instead, using the Fullband profile.
* Channels, sample rate, etc.: All audio streams will be included in the encoded media, using the same channel layout (7, 5.1, stereo, etc.) and sample rate as the original. The only exception to this is when the original media contains 7.1 channel DTS streams (e.g. from BluRay discs). As some of my existing devices [only support 5.1 channels in Vorbis streams](https://developer.samsung.com/smarttv/develop/specifications/media-specifications/2018-tv-video-specifications.html) these have to be converted to 5.1 channel streams, otherwise playback is impossible to watch due to its constant buffering and stuttering. If my devices _did_ support Opus, then this utility would also change `5.1(side)` streams (commonly found on DVD media) to `5.1` due to an [FFmpeg bug](https://trac.ffmpeg.org/ticket/5718).
* Metadata: Language metadata will be added to each audio stream; this helps media players automatically select the correct stream based on a user's language preference settings.

### Subtitles

* All subtitles are currently skipped/stripped. Whilst retaining them would be in-keeping with `webmencoder`s general principle of preserving as much of the original media's contents and settings as possible, I'm yet to find a suitable open source tool that can extract subtitles from an MPEG file, OCR them, and convert them to `WebVTT` format for inclusion into the WebM container. If anyone knows of such a tool, I'll be happy to add subtitle support!

## Usage

```bash
USAGE:
    webmencoder [OPTIONS] --input-dir <input-dir> --output-dir <output-dir>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --hw-device <hw-device>       [default: /dev/dri/renderD128]
    -i, --input-dir <input-dir>
    -o, --output-dir <output-dir>
    -q, --quality <quality>           [default: 100]
```

As you can see, running `webmencoder` is rather trivial. You simply give it an input directory and an output directory and it will recursively process all `.mpeg` files it finds and convert them to `.webm` files. You can optionally provide it with a quality setting, if you find the default to be either too poor in quality or results in files larger than you'd prefer. It's also possible to tell it what hardware device to use to perform the encoding in case the default isn't suitable.

## TODO

* Error handling
* Allow audio stream id -> language mappings to be passed in at runtime instead of being hardcoded; these are unlikely to be consistent across input files

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
