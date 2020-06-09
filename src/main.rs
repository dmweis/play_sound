use clap::Clap;
use rodio::{self, DeviceTrait};
use std::io::BufReader;

#[derive(Clap)]
#[clap(
    version = "0.1.0",
    author = "David M. Weis <dweis7@gmail.com>",
    about = "Simple CLI tool for playing audio files using selected audio device"
)]
struct Opts {
    #[clap(
        short = "d",
        long = "device",
        about = "Name of desired audio output device"
    )]
    output_device: Option<String>,
    #[clap(
        short = "f",
        long = "file",
        required_unless = "list_devices",
        conflicts_with = "list_devices",
        about = "Path to audio file"
    )]
    file: Option<String>,
    #[clap(
        name = "list_devices",
        short = "l",
        long = "list",
        about = "Lists all available devices and exits"
    )]
    list_devices: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    if opts.list_devices {
        let audio_outputs = rodio::devices().expect("failed enumerating devices");
        for output in audio_outputs {
            println!("{}", output.name().expect("Failed getting name for device"));
        }
        std::process::exit(0);
    }
    let mut output_device = rodio::default_output_device().expect("Failed getting default device");
    if let Some(device_name) = opts.output_device {
        let audio_outputs = rodio::devices().expect("Failed getting name for device");
        for output in audio_outputs {
            if device_name == output.name().expect("Failed getting default device") {
                output_device = output;
            }
        }
    }
    if output_device.default_output_format().is_ok() {
        let output_sink = rodio::Sink::new(&output_device);
        if let Some(file_path) = opts.file {
            if let Ok(file) = std::fs::File::open(file_path) {
                output_sink.append(rodio::Decoder::new(BufReader::new(file)).expect("Failed accessing output device"));
                output_sink.sleep_until_end();
            } else {
                eprintln!("Failed reading file");
                std::process::exit(1);
            }
        } else {
            eprintln!("Please provide file you want to play");
            std::process::exit(1);
        }
    } else {
        eprintln!("Selected device doesn't support output");
        std::process::exit(1);
    }
}
