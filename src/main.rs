// DIRP - a directory enumerating tool
// Written by Simeon Duwel

// This is probably very poorly written. I am not an experienced Rust dev by any means.
// If you feel like improving on this, feel free to open a PR.

use structopt::StructOpt;
use termion::color;
use std::time::Instant;
use hhmmss::Hhmmss;
use std::time::Duration;
use std::thread;

#[derive(Debug, StructOpt)]
#[structopt(name = "dirp", about = "a directory enumerating tool", author = "Simeon Duwel")]
struct Cli {
    #[structopt(parse(from_os_str), short, long, default_value = "/home/simeon/useful/wordlists/dirbuster-medium.txt")]
    wordlist: std::path::PathBuf,

    #[structopt(short = "4", long)]
    display_404s: bool,

    #[structopt(short = "T", long)]
    display_time: bool,
    
    #[structopt(short = "r", long, default_value = "0.0")]
    rate: f32,
    
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let greeter = "
       __ _            
  ____/ /(_)_____ ____ 
 / __  // // ___// __ \\
/ /_/ // // /   / /_/ /
\\__,_//_//_/   / .___/ 
              /_/  
    ";
    println!("{}{}{}", color::Fg(color::Cyan), greeter, color::Fg(color::Reset));

    let args = Cli::from_args();

    let wordlist = std::fs::read_to_string(&args.wordlist).expect("Could not read the provided word list.");
    let target = args.target.to_owned();
    
    let rate: u64 = if args.rate == 0.0 {0} else {(1000.0/args.rate) as u64};

    for dir in wordlist.lines() {
        if dir.starts_with("#") {
            continue;
        }

        let full_path = target.clone() + "/" + dir;
        let res = reqwest::get(&full_path).await?;
        let status = res.status();
        
        if args.display_time {
	        print!("{}[ {} ]{} ", color::Fg(color::Magenta), start_time.elapsed().hhmmssxxx(), color::Fg(color::Reset));
	    }
	
	    match status.as_u16() {
            200 => println!("{}{}: {}{}", color::Fg(color::LightGreen), &full_path, status, color::Fg(color::Reset)),
            301 => println!("{}{}: {}{}", color::Fg(color::Yellow),     &full_path, status, color::Fg(color::Reset)),
            404 => {
                if args.display_404s {
                    println!("{}{}{}", color::Fg(color::LightBlack),     &full_path, color::Fg(color::Reset));
                }
            },
            _   => println!("{}: {}", &full_path, status), 
        }

        thread::sleep(Duration::from_millis(rate));
    }
    Ok(())
}
