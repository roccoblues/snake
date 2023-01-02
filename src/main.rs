use clap::Parser;

mod cli;
mod input;
mod output;
mod path;
mod snake;
mod types;

fn main() {
    let opts = cli::Opts::parse();

    output::init();

    let config = snake::Config::from(opts);
    snake::run(&config);

    output::reset();
}
