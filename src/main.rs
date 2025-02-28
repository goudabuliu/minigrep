use std::env;
use std::process;
fn main() {
    // let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let cfg= minigrep::Cfg::build(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });
    // println!("{}", cfg.query);
    // println!("{}", cfg.file);
    if let Err(e) =minigrep::run(&cfg) {
        eprintln!("App error\n{}", e);
        process::exit(1);
    }
}


