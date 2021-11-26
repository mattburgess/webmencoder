// SPDX-License-Identifier: (MIT OR Apache-2.0)

use glob::glob;
use serde::Deserialize;
use std::process::Command;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "webmencoder",
    about = "ffmpeg wrapper to convert MPEG videos to WebM"
)]
struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    input_dir: std::path::PathBuf,
    #[structopt(short, long, parse(from_os_str))]
    output_dir: std::path::PathBuf,
    #[structopt(short, long, default_value = "/dev/dri/renderD128")]
    hw_device: String,
    #[structopt(short, long, default_value = "100")]
    quality: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "codec_type")]
enum Stream {
    Video {
        field_order: String,
    },
    Audio {
        #[serde(default = "default_channel_layout")]
        channel_layout: String,
        channels: u8,
        id: String,
    },
    Subtitle,
    Data,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    streams: Vec<Stream>,
}

fn default_channel_layout() -> String {
    "stereo".to_string()
}

fn get_lang_from_id(id: &str) -> &str {
    match id {
        "0x80" => "eng",
        "0x81" => "fra",
        "0x82" => "spa",
        _ => "",
    }
}

fn build_command(
    metadata: &Metadata,
    hw_device: &str,
    quality: u8,
    input_vid: &std::path::Path,
    output_dir: &std::path::Path,
) -> Result<(), std::io::Error> {
    let mut a_filters = Vec::new();
    let mut mappings = Vec::new();
    let mut a_metadata = Vec::new();
    let (mut v_idx, mut a_idx) = (0, 0);
    let mut v_filters = vec!["format=nv12", "hwupload"];
    for stream in metadata.streams.iter() {
        match stream {
            Stream::Audio {
                channel_layout,
                channels,
                id,
            } => {
                if *channels == 0 {
                    mappings.push("-map".to_string());
                    mappings.push(format!("-0:a:{}", a_idx));
                } else if channel_layout == "7.1" {
                    {
                        a_filters.push(format!(
                            "[0:a:{}]aformat=channel_layouts='5.1'['audio{}']",
                            a_idx, a_idx
                        ));
                        mappings.push("-map".to_string());
                        mappings.push(format!("[audio{}]", a_idx));
                    }
                } else {
                    mappings.push("-map".to_string());
                    mappings.push(format!("0:a:{}", a_idx));
                }
                a_metadata.push(format!("-metadata:s:a:{}", a_idx));
                a_metadata.push(format!("language={}", get_lang_from_id(id)));
                a_idx += 1;
            }

            Stream::Video { field_order } => {
                if ["tt".to_string(), "tb".to_string(), "bt".to_string()].contains(field_order) {
                    v_filters.insert(0, "yadif");
                }
                mappings.push("-map".to_string());
                mappings.push(format!("0:v:{}", v_idx));
                v_idx += 1;
            }

            Stream::Subtitle => {}

            Stream::Data => {}
        }
    }

    let in_file = input_vid.to_str().unwrap();
    let mut out_path = output_dir.join(input_vid);
    out_path.set_extension("webm");
    let out_file = out_path.to_str().unwrap();
    let a_filter = a_filters.join(";");
    let v_filter = v_filters.join(",");
    let mut ffmpeg = Command::new("ffmpeg");
    let mut ffmpeg_args = vec!["-vaapi_device", hw_device, "-i", in_file];

    if !&a_filter.is_empty() {
        ffmpeg_args.extend_from_slice(&["-filter_complex", &a_filter])
    }

    let mapping_args: Vec<&str> = mappings.iter().map(AsRef::as_ref).collect();

    ffmpeg_args.extend_from_slice(&mapping_args);

    let metadata_args: Vec<&str> = a_metadata.iter().map(AsRef::as_ref).collect();

    ffmpeg_args.extend_from_slice(&metadata_args);

    let qual = &quality.to_string();
    ffmpeg_args.extend_from_slice(&[
        "-vf",
        &v_filter,
        "-c:v",
        "vp9_vaapi",
        "-global_quality",
        qual,
        "-c:a",
        "libvorbis",
        "-y",
        out_file,
    ]);
    ffmpeg.args(ffmpeg_args);
    println!("{:?}", ffmpeg);
    ffmpeg.status().expect("Failed to convert video");
    Ok(())
}

fn get_video_details(input_vid: &std::path::Path) -> Result<Metadata, std::io::Error> {
    let ffprobe = Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-of",
            "json",
            "-show_streams",
            input_vid.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to analyze input file");
    let parsed_output: Metadata = serde_json::from_slice(&ffprobe.stdout)?;
    Ok(parsed_output)
}

fn convert_file(
    input_vid: &std::path::Path,
    output_dir: &std::path::Path,
    hw_device: &str,
    quality: u8,
) -> Result<(), std::io::Error> {
    let vid_details = get_video_details(input_vid).unwrap();
    build_command(&vid_details, hw_device, quality, input_vid, output_dir)
}

fn convert_files(
    input_dir: &std::path::Path,
    output_dir: &std::path::Path,
    hw_device: &str,
    quality: u8,
) -> Result<(), std::io::Error> {
    let pattern = format!("{}{}", input_dir.to_str().unwrap(), "/**/*.mpeg");
    for entry in glob(&pattern).unwrap() {
        match entry {
            Ok(path) => convert_file(&path, output_dir, hw_device, quality).unwrap(),
            Err(e) => return Err(e.into_error()),
        }
    }
    Ok(())
}
fn main() {
    let args = Cli::from_args();
    convert_files(
        &args.input_dir,
        &args.output_dir,
        &args.hw_device,
        args.quality,
    )
    .unwrap();
}
