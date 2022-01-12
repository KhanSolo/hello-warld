use clap::{App, Arg};

fn main() {
    
    let matches = App::new("echor")
        .version("0.1.0")
        .author("KonstT")
        .about("rust echo")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
            .help("Do not print newline")
            .takes_value(false)
            .short('n'),
        )
        .get_matches();

    println!("{:#?}", matches);
}
