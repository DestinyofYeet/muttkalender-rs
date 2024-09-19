use clap::Parser;

mod file_reader;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Print out ics files", long_about = None)]
struct Args {
    #[arg(short = 'i', long = "input")]
    input: String,

    #[arg(short = 'c', long = "charset")]
    charset: Option<String>,

    #[arg(short = 'v', long = "verbose", default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let content = file_reader::read_file(args.input, args.charset);

    let lines = content.split('\n');

    //unfold file

    for line in lines.into_iter() {
        let split: Vec<&str> = line.split(':').collect();

        println!("Left: {} | Right: {}", split[0], split[1]);
    }

    println!("File content: \n{}", content);
}
