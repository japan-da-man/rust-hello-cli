use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rust-hello-cli",
    version = "1.0.0",
    author = "japan-da-man",
    about = "A command-line application for parctice"
)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[arg(name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.formula_file {
        Some(file) => println!("File specificed: {}", file),
        None => println!("No file specified"),
    }
    println!("Is verbosity specisty?: {}", cli.verbose);
}
