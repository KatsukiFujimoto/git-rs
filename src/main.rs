use git_rs::cli::app::App;
use std::env::current_dir;

fn main() -> anyhow::Result<()> {
    let current_dir = current_dir()?;
    App::new().start(&current_dir.as_path())?;
    Ok(())
}
