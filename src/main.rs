use std::{
    fs,
    io::stdout,
    path::PathBuf,
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::{Print, Stylize},
    terminal::{Clear, ClearType},
};

#[derive(Parser, Debug)]
struct Cli {
    #[doc = "Path to file that contain text"]
    #[clap(short, long)]
    file: PathBuf,
    #[doc = "Count of written characters that required to get results. If count == 0, whole text used. If len(characters) < count, same text used repeatly"]
    #[clap(short, long, default_value_t = 0)]
    count: usize,
}

impl Cli {
    fn get_file_contents(&self) -> Result<String, String> {
        String::from_utf8(fs::read(&self.file).map_err(|_| {
            format!(
                "Cannot read file '{}'",
                self.file.as_os_str().to_string_lossy()
            )
        })?)
        .map_err(|_| "Cannot convert file contents into valid UTF-8 string".into())
    }
}

enum State {
    PressEnterToStart,
    PreGame,
    Game { start_time: u64, pos: usize },
}

fn get_current_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn play(text: &str) {
    let mut state = State::PressEnterToStart;
    loop {
        match state {
            State::PressEnterToStart => {
                execute!(
                    stdout(),
                    Clear(ClearType::All),
                    MoveTo(0, 0),
                    Print("GorillaType v0.1"),
                    MoveTo(0, 2),
                    Print(format!("Press {} to start", "Space".blue()))
                )
                .unwrap();
                if matches!(event::read().unwrap(), Event::Key(key) if key.code == KeyCode::Char(' '))
                {
                    state = State::PreGame;
                }
            }
            State::PreGame => {
                execute!(
                    stdout(),
                    Clear(ClearType::All),
                    MoveTo(0, 0),
                    Print("GorillaType v0.1"),
                    MoveTo(0, 2),
                    Print(text.dark_grey()),
                    MoveTo(0, 2)
                )
                .unwrap();
                let current_char = text.chars().next().unwrap();
                if matches!(event::read().unwrap(), Event::Key(key) if key.code == KeyCode::Char(current_char))
                {
                    execute!(stdout(), Print(current_char.to_string().green())).unwrap();
                    state = State::Game {
                        start_time: get_current_seconds(),
                        pos: 1,
                    };
                }
            }
            State::Game {
                start_time,
                ref mut pos,
            } => {
                let current_char = text.chars().nth(*pos).unwrap();
                if matches!(event::read().unwrap(), Event::Key(key) if key.code == KeyCode::Char(current_char))
                {
                    execute!(
                        stdout(),
                        MoveTo(*pos as u16, 2),
                        Print(current_char.to_string().green())
                    )
                    .unwrap();
                    *pos += 1;
                }
            }
        }
    }
}

fn main0() -> Result<(), String> {
    let cli = Cli::parse();
    let mut text = cli.get_file_contents()?;
    if cli.count != 0 {
        if text.len() < cli.count {
            let source = text.clone();
            let text_repeat = (cli.count - source.len()) / source.len();
            text.push_str(&[" ", &source].concat().repeat(text_repeat));
            text.push_str(&[" ", &source[..(cli.count - text.len() - 1)]].concat());
        } else if text.len() > cli.count {
            text = [" ", &text[..cli.count]].concat();
        }
    }
    play(text.as_str());
    Ok(())
}

fn main() {
    drop(main0().inspect_err(|err| {
        eprintln!("{err}");
        exit(1)
    }));
}
