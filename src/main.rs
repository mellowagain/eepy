use clap::Parser;

fn main() {
    let a = Arguments::parse();
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[clap(subcommand)]
    command: Option<SubCommands>
}

#[derive(Debug)]
enum SubCommands {

}
