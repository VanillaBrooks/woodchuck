use clap::{Arg, Command};
use woodchuck::Headers;
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
    let table_format = matches.value_of("table-format").unwrap();
    let table_args = matches.value_of("table-args").unwrap();
    let multirow_headers: bool = matches.is_present("multi-row-headers");

    // open the input file
    let f = std::fs::File::open(input).expect("Could not open the input csv");
    let mut table = LatexTable::from_reader(
        f,
        caption,
        label,
        table_format,
        table_args,
        Headers::from_cli_bool(multirow_headers),
    );

    // open the tex file to write to
    let mut writer =
        std::fs::File::create(output).expect("Could not open / create the output file");

    table
        .to_writer(&mut writer)
        .expect("Could not write the result to file");
}

fn make_cli() -> Command<'static> {
    Command::new("Woodchuck")
        .version("0.2.0")
        .author("Brooks")
        .about("Convert CSV files to latex tables")
        .arg(
            Arg::new("csv")
                .value_name("FILE")
                .help("Input path to csv")
                .takes_value(true),
        )
        .arg(
            Arg::new("latex")
                .value_name("FILE")
                .help("Path to .tex file to write to")
                .takes_value(true),
        )
        .arg(
            Arg::new("caption")
                .long("caption")
                .default_value("Caption Here"),
        )
        .arg(
            Arg::new("label")
                .long("label")
                .default_value("Label Here"),
        )
        .arg(
            Arg::new("table-format")
                .long("table-format")
                .default_value("")
                .help("value `v` is placed in the header as \\begin{table`v`}. Usually it can be `*` to make the table full-page width")
        )
        .arg(
            Arg::new("table-args")
                .long("table-args")
                .default_value("[H]")
        )
        .arg(
            Arg::new("multi-row-headers")
                .long("multi-row-headers")
                .short('m')
                .help("use organization of first _two_ rows to construct the headers for the table")
                .takes_value(false)
                .required(false)
        )
}
