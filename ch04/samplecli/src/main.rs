use clap::{App, Arg};

#[derive(Clap, Debug)]
#[clap(
    name="My PRN program",
    version="1.0.0",
    name= "KUrushi",
    about = "Super awesome sample PRN calculator"
)]

struct Opts {
    /// Sets the level of verbosity
    #[clap(short, long)]
    verbose: bool,
    /// Formulas written in PRN
    #[clap(name="FILE")]
    formula_file = Option<String>,
}

fn main() {
    let opts = Opts::parse();
    match opts.formula_file {
        Some(file) => println!("File specified:{}", file);
        None => println!("No file specified."),
    }

    println!("Is verbosity specified?: {}", opts.verbose);
:wq

}
