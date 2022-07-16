use git_rs::{app::App, cli::app::App as CliApp};
use std::{env::current_dir, io};

fn main() -> anyhow::Result<()> {
    let current_dir = current_dir()?;
    CliApp::start(&current_dir.as_path());
    Ok(())
}

fn main2() -> Result<(), io::Error> {
    let res = App::start();
    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}
