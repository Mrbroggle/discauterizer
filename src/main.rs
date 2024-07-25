use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::Split;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //Path to the input file
    #[clap(short, long, value_name = "FILE")]
    input: PathBuf,

    //Output size in MB
    #[clap(short, long, default_value = "500")]
    size: u32,

    //Bake subtitles 1 = yes, 0 = no
    #[clap(short, long, default_value = "1")]
    bake: u32,

    //Verbose output 1 = yes, 0 = no
    #[clap(short, long, default_value = "0")]
    verbose: u32,

    //Leeway in the output size in MB
    #[clap(short, long, default_value = "50")]
    leeway: u32,

    //Path to the output file
    #[clap(short, long, default_value = "output.mp4", value_name = "FILE")]
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

    let leewayed_size = (args.size - args.leeway) as f32 * 8000000.0;

    let bitrate = probed.next().unwrap();
    let calculated_bitrate = (leewayed_size / duration).to_string();
    let str_bitrate = calculated_bitrate.as_str();

    println!(
        "Original Bitrate = {}, Calculated Bitrate = {}, Duration = {}, Size = {}, Output = {}",
        bitrate,
        str_bitrate,
        duration,
        args.size,
        args.output.to_str().unwrap()
    );
    let mut sub_args = vec!["-i", &args.input.as_os_str().to_str().unwrap(), "lmao.ass"];

    if &args.verbose.clone().to_string() == "0" {
        sub_args.append(&mut vec!["-v", "quiet", "-stats"]);
    };

    let subtitle = Command::new("ffmpeg")
        .args(sub_args)
        .spawn()
        .expect("Failed to create subtitles");

    println!("{:?}", subtitle.wait_with_output());

    let encode_args = vec![
        "-i",
        &args.input.as_os_str().to_str().unwrap(),
        "-vf",
        "ass=lmao.ass",
        "-b:v",
        str_bitrate,
        &args.output.as_os_str().to_str().unwrap(),
    ];

    let verbose = vec!["-v", "quiet", "-stats"];

    let mut encode_args_verbose: Vec<&str> = vec![];

    if &args.verbose.clone().to_string() == "0" {
        encode_args_verbose.extend(verbose.iter().cloned());
        encode_args_verbose.extend(encode_args.iter().cloned());
    } else {
        encode_args_verbose.extend(encode_args.iter().cloned());
    };
    //Run ffmpeg with the calculated bitrate
    let ffmpeg = Command::new("ffmpeg")
        .args(encode_args_verbose)
        .spawn()
        .expect("Failed to execute ffmpeg");

    println!("{:?}", ffmpeg.wait_with_output());
    fs::remove_file("lmao.ass").unwrap();
}
