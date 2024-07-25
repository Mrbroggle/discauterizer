use clap::Parser;
use std::path::PathBuf;
use std::process::Command;
use std::str::Split;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Path to the input file
    #[clap(short, long)]
    input: PathBuf,

    //Output size in MB
    #[clap(short, long, default_value = "500")]
    size: u32,

    //Leeway enabled
    #[clap(short, long)]
    leeway_enabled: bool,

    //Leeway in the output size in MB
    #[clap(short, long, default_value = "10")]
    leeway: u32,

    //Path to the output file
    #[clap(short, long, default_value = "output.mp4")]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();

    //Get the duration and bitrate of the input file
    let ffprobe = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration:format=bit_rate")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(&args.input)
        .output()
        .expect("Failed to execute ffprobe");

    let bound_probed = String::from_utf8_lossy(&ffprobe.stdout);
    let mut probed: Split<&str>;
    if cfg!(target_os = "windows") {
        probed = bound_probed.split("\r\n");
    } else {
        probed = bound_probed.split("\n")
    }
    let duration: f32 = probed
        .next()
        .unwrap()
        .parse::<f32>()
        .expect("Failed to parse duration");

    let leewayed_size = if args.leeway_enabled {
        (args.size - args.leeway) as f32 * 8000000.0
    } else {
        args.size as f32 * 800000.0
    };

    let bitrate = probed.next().unwrap();
    let calculated_bitrate = leewayed_size / duration;

    println!(
        "Original Bitrate = {}, Calculated Bitrate = {}, Duration = {}, Size = {}, Output = {}",
        bitrate,
        calculated_bitrate,
        duration,
        args.size,
        args.output.to_str().unwrap()
    );

    //Run ffmpeg with the calculated bitrate
    let ffmpeg = Command::new("ffmpeg")
        .arg("-i")
        .arg(&args.input)
        .arg("-b")
        .arg(calculated_bitrate.to_string())
        .arg(&args.output)
        .spawn()
        .expect("Failed to execute ffmpeg");

    println!("{:?}", ffmpeg);
}
