// 1. pretty print

// fn main() {
//     // pretty print
//     println!("{:?}", std::env::args());
// }

// cargo run -- -n Hello world
// -- meaning seprate Cargo's optpions
// will return
// Args { inner: ["target/debug/echor", "-n", "Hello", "world"] }

// use  clap (command-line argument parser) crate

// 2. Basic add some options to --help

use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Josh Chou")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newlline")
                .short("n")
                .help("Do not print newlline")
                .takes_value(false),
        )
        // tell the App to parse the arguments
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });

    // println!("{:#?}", matches)
}
