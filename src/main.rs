use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

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

fn play(text: &str) {
    loop {
        
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
