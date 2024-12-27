// for command line
use clap;
// for time and sleep
use std::{thread, time};
// for error
use std::io;
// for audio
use std::fs::File;
use rodio::{Decoder, OutputStream, source::Source};
// for clean exit
use std::process;
// for picking a random music
use rand::Rng;

fn main() {

    // set up the input command line
    let args = clap::Command::new("pomo")
        .author("shubham, shubham@gar.gs")
        .version("0.0.1")
        .about("Plays a audio-visual indicator after set time elapses.")
        .arg(clap::arg!(<TIME> "set the time for the pomodoro period").default_value("25"))
        .arg(clap::arg!(-s --sound <SOUND> "set the music file to play after timer ends"))
        .get_matches();

    // check if enetered time is valid
    let time_to_sleep = match get_time_from_cmd_arg(args.get_one::<String>("TIME")) {
        Ok(data) => data,
        Err(_) => {
            println!("Value given {:?} is not a valid number", args.get_one::<String>("TIME").unwrap());
            process::exit(1);
        },
    };

    // select random music file
    let mut rng = rand::thread_rng();
    let alarm_file = match args.get_one::<String>("sound") {
        None => format!("/home/gshubham96/git/pomo/src/media/alarm_{}.mp3", rng.gen_range(1..=2)),
        Some(file) => file.to_string(),
    };

    // sleep if appropriate
    if time_to_sleep < 100.0 {
        sleep_for_n_minutes(time_to_sleep);
        play_sound_on_finish(&alarm_file); 
    } else {
        println!("Value enetered {time_to_sleep} is kinda crazy!.");
        process::exit(1);
    }
    process::exit(0);
}

/// reads the cmdline arg and converts to an unsigned integer
fn get_time_from_cmd_arg(arg: Option<&String>) -> io::Result<f32> {
    match arg {
        None => Ok(25.0),
        Some(text) => {
            match text.parse::<f32>() {
                Ok(time) => Ok(time),
                Err(_) => Err(io::Error::from(io::ErrorKind::InvalidData)),
            }
        }
    }
}

/// sleeps for n minutes (1st implementation)
fn sleep_for_n_minutes(mut n: f32) {
    loop {
        let mins: u32 = n.floor() as u32;
        let secs: u32 = ((n - n.floor()) * 60.0) as u32;
        print_remaining_time(mins, secs);
        thread::sleep(time::Duration::from_secs(1));
        n = n - 0.0167;

        // exit condition
        if n < 0.0167 {
            print_timer_end();
            break;
        }
    };
}

fn print_remaining_time(min: u32, sec: u32) {
    let colors = [
        "\x1b[38;5;226m", // Light Yellow
        "\x1b[38;5;214m", // Orange
        "\x1b[38;5;208m", // Light Orange
        "\x1b[38;5;202m", // Peach
        "\x1b[91m",  // Light Red
        "\x1b[31m",  // Red
        "\x1b[38;5;196m", // Bright Red
        "\x1b[38;5;124m"  // Dark Red
    ];
    print!("{}[2J", 27 as char);   
    for color in colors.iter().rev() {
        println!("{}{}\x1b[0m", color, "###################################################");
    }

    println!("\x1b[1;93m{}\x1b[0m", "###-------------- Remaining Time ---------------###");
    println!("\x1b[1;93m###--------------      {:02}:{:02}     ---------------###\x1b[0m", min, sec);
    println!("\x1b[1;93m{}\x1b[0m", "###---------------------------------------------###");

    for color in colors.iter() {
        println!("{}{}\x1b[0m", color, "###################################################");
    }

}

fn print_timer_end() {
    let colors = [
        "\x1b[38;5;226m", // Light Yellow
        "\x1b[38;5;214m", // Orange
        "\x1b[38;5;208m", // Light Orange
        "\x1b[38;5;202m", // Peach
        "\x1b[91m",  // Light Red
        "\x1b[31m",  // Red
        "\x1b[38;5;196m", // Bright Red
        "\x1b[38;5;124m"  // Dark Red
    ];
    print!("{}[2J", 27 as char);   
    for color in colors.iter().rev() {
        println!("{}{}\x1b[0m", color, "###################################################");
    }

    println!("\x1b[1;93m{}\x1b[0m", "###---------------------------------------------###");
    println!("\x1b[1;93m###-----------   !!TIMER FINISHED!! ------------###\x1b[0m");
    println!("\x1b[1;93m{}\x1b[0m", "###---------------------------------------------###");

    for color in colors.iter() {
        println!("{}{}\x1b[0m", color, "###################################################");
    }

}

/// plays a set sound
fn play_sound_on_finish(audio_file: &str) {  
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = io::BufReader::new(File::open(audio_file).unwrap());
    let source = Decoder::new(file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    thread::sleep(time::Duration::from_secs(16));
}