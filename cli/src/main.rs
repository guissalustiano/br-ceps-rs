use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Brazil postal code
    cep: String,
}

fn main() {
    let args = Args::parse();

    let r = br_ceps::get(&args.cep).unwrap();
    println!("{:#?}", r);
}
