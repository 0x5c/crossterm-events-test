use std::time::Duration;

use crossterm::{
    event,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
    },
    style::Stylize,
    tty::IsTty,
};


fn main() {
    let stdout = std::io::stdout();
    
    if !stdout.is_tty() {
        eprintln!("{}", "Must be run in a TTY!");
        std::process::exit(1);
    }
    
    let matches = clap::App::new(clap::crate_name!())
                                .version(clap::crate_version!())
                                .author(clap::crate_authors!())
                                .about(clap::crate_description!())
                                .arg(clap::Arg::with_name("wait")
                                    .long("wait")
                                    .short("w")
                                    .takes_value(true)
                                    .value_name("MICROSECONDS")
                                    .default_value("2000000")
                                    .help("Duration of the wait period"))
                                .arg(clap::Arg::with_name("poll")
                                    .long("poll")
                                    .short("p")
                                    .takes_value(true)
                                    .value_name("MICROSECONDS")
                                    .default_value("1000000")
                                    .help("Duration of the polling period"))
                                .get_matches();

    let wait_time = match matches.value_of("wait").unwrap().parse::<u64>() {
        Result::Ok(n) => std::time::Duration::from_micros(n),
        Result::Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(1)
        }
    };
    let poll_time = match matches.value_of("poll").unwrap().parse::<u64>() {
        Result::Ok(n) => std::time::Duration::from_micros(n),
        Result::Err(err) => {
            eprintln!("Error: {:?}", err);
            std::process::exit(1)
        }
    };

    match loop_de_loop(wait_time, poll_time) {
        crossterm::Result::Ok(_) => {
            disable_raw_mode().unwrap();
            std::process::exit(0)
        },
        crossterm::Result::Err(err) => {
            eprintln!("Error: {:?}", err);
            disable_raw_mode().unwrap();
            std::process::exit(1)
        },
    }
}


fn loop_de_loop(wait_dur: Duration, poll_dur: Duration) -> crossterm::Result<()> {
    let ctrl_c = event::Event::Key(event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL)); 
    enable_raw_mode()?;
    loop {
        println!("{}\r", "Waiting period".dark_green());
        std::thread::sleep(wait_dur);
        println!("{}\r", "Polling period".dark_yellow());
        match event::poll(poll_dur)? {
            true => {
                let event = event::read()?;
                let fmt_event = format!("{:?}", event).blue();
                println!("{} {}\r", "Got Event:".dark_cyan(), fmt_event);
                if event == ctrl_c {
                    println!("{}\r", "Ctrl+C means we say byebye!".dark_cyan());
                    break;
                }
            },
            false => println!("{}\r", "Got no event".red()),
        }
    }
    Ok(())
}
