mod app;
use crate::app::App;
use std::io;
use std::process::Command;

#[derive(Default, Debug)]
pub struct Tree {
    pub files: Vec<String>,
}

impl Tree {
    pub fn get_files(&mut self) {
        if cfg!(target_os = "linux") {
            // NOTE: check Command's env methods
            let out = Command::new("ls")
                .arg("-p")
                .output()
                .expect("failed to exec process");
            if out.status.success() {
                // println!("{}", String::from_utf8_lossy(&out.stdout));
                let output = String::from_utf8(out.stdout);
                match output {
                    Ok(files) => {
                        self.files = files
                            .split("\n")
                            .map(|s| s.to_string())
                            .filter(|s| !s.is_empty())
                            .collect()
                    }
                    Err(err) => println!("UTF8Error: {}", err),
                }
            } else {
                eprintln!(
                    "Command failed with error: {}",
                    String::from_utf8_lossy(&out.stderr)
                );
            }
        } else {
            eprintln!("Only runs on linux L");
        }
    }
}
fn main() -> io::Result<()> {
    let mut tree = Tree::default();
    tree.get_files();
    println!("{:?}", tree);
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
