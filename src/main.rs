use std::{fs, path::PathBuf, process::exit};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[doc = "Path to file that contain text"]
    #[clap(short, long)]
    file: PathBuf,
    #[doc = "Count of written words that required to get results. If len(words) < count, same text used"]
    #[clap(short, long, default_value_t = 100)]
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

    fn get_count(&self) -> Result<usize, String> {
        if self.count == 0 {
            Err("Count must be positive integer")?
        }
        Ok(self.count)
    }
}

fn main0() -> Result<(), String> {
    let cli = Cli::parse();
    let text = cli.get_file_contents()?;
    let count = cli.get_count()?;
    Ok(())
}

fn main() {
    drop(main0().inspect_err(|err| {
        println!("{err}");
        exit(1)
    }));
}
