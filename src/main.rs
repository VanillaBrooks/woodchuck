use clap::{App, Arg};
use woodchuck::LatexTable;

fn main() {
    let app = make_cli();
    let matches = app.get_matches();

    let input = if let Some(x) = matches.value_of("csv") {
        x
    } else {
        panic!("Was expecting a path to csv input file")
    };

    let output = if let Some(x) = matches.value_of("latex") {
        x
    } else {
        panic!("Was expecting a path to csv input file")
    };

    let caption = matches.value_of("caption").unwrap();
    let label = matches.value_of("label").unwrap();

    // open the input file
    let f = std::fs::File::open(input).expect("Could not open the input csv");
    let mut table = LatexTable::from_reader(f, caption, label);

    // open the tex file to write to
    let mut writer =
        std::fs::File::create(output).expect("Could not open / create the output file");

    table
        .to_writer(&mut writer)
        .expect("Could not write the result to file");
}

fn make_cli() -> App<'static, 'static> {
    App::new("Woodchuck")
        .version("0.1.0")
        .author("Brooks")
        .about("Convert CSV files to latex tables")
        .arg(
            Arg::with_name("csv")
                .value_name("FILE")
                .help("Input path to csv")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("latex")
                .value_name("FILE")
                .help("Path to .tex file to write to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("caption")
                .long("caption")
                .default_value("Caption Here"),
        )
        .arg(
            Arg::with_name("label")
                .long("label")
                .default_value("Label Here"),
        )
}
