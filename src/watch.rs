use conf;
use console::{style, Emoji};
use indicatif::ProgressBar;
use notify::DebouncedEvent;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch() {
    println!("watch...");
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher
        .watch(conf::EXERCISE_PATH, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Chmod(path) | DebouncedEvent::Write(path) => {
                    compile_exercise(path);
                }
                _ => {}
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn compile_exercise(path: PathBuf) {
    // 1.get new file name and verify
    let filename = path.file_name().unwrap().to_str().unwrap();
    if !(filename.starts_with(conf::FILE_PREFIX) && filename.ends_with(conf::FILE_SUFFIX)) {
        println!("add useless file {}, ignore it!", filename);
        return;
    }

    // judge if need performer unit test
    let file_path = format!("{}/{}", conf::EXERCISE_PATH, filename).to_owned();
    let content = fs::read_to_string(file_path).unwrap();
    if !content.contains(conf::COMPILE_FLAG) {
        return;
    }

    // init bar
    let bar = ProgressBar::new_spinner();
    bar.set_message(format!("Compiling {}...", filename).as_str());
    bar.enable_steady_tick(100);

    // performer uint test
    let package = format!("exercises::{}", filename.replace(conf::FILE_SUFFIX, "")).to_owned();
    let compile_cmd = Command::new("cargo")
        .args(&["test", &package])
        .output()
        .expect("fail");
    bar.finish_and_clear();
    if compile_cmd.status.success() {
        let format_str = format!(
            "{} Successfully compiled {}!",
            Emoji("✅", "✓"),
            filename
        );
        println!("{}", style(format_str).green());
    } else {
        let format_str = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(format_str).red());
        let stderr = String::from_utf8_lossy(&compile_cmd.stderr);
        if !stderr.contains("error: test failed") {
            println!(
                "{}",
                style(
                    Regex::new("warning: An explicit([\\s\\S]*)please set bin.path in Cargo.toml")
                        .unwrap()
                        .replace_all(&stderr, "")
                        .to_string()
                )
                .red()
                .underlined(),
            );
        }
        if compile_cmd.stdout.len() != 0 {
            println!(
                "{}",
                style(String::from_utf8_lossy(&compile_cmd.stdout))
                    .red()
                    .underlined()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        println!("{}", style("123").magenta());
        assert_eq!(
            Regex::new("warning([\\s\\S]*)Cargo.toml")
                .unwrap()
                .replace_all("12 32 warning 123 \n ada --> Cargo.toml hello", "")
                .to_string(),
            String::from("12 32  hello")
        );
    }
}
