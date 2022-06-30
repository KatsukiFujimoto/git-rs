use git_rs::app::App;
use std::io;

fn main() -> Result<(), io::Error> {
    let res = App::start();
    if let Err(err) = res {
        println!("{:?}", err);
    }
    Ok(())
}
