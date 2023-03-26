use clap::{value_parser, Arg, Command};
use keyberon_layout_serde::qmk::QmkKeymap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

/// Build a Command
fn build_cli() -> Command {
    Command::new("qmk-layout-to-keyberon")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Boris Faure <boris@fau.re>")
        .about("Convert a QMK layout JSON file into a Keyberon layout rust file")
        .arg(
            Arg::new("json")
                .value_name("QMK_JSON")
                .num_args(1)
                .required(true)
                .help("a QMK layout JSON file")
                .value_parser(value_parser!(PathBuf)),
        )
}

fn main() {
    lovely_env_logger::init(lovely_env_logger::Config::new_reltime());

    let matches = build_cli().get_matches();
    let json_path = matches.get_one::<PathBuf>("json").unwrap();
    let mut s = String::new();
    File::open(json_path)
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let res = QmkKeymap::from_json_str(&s);
    let keymap = res.unwrap();
    println!("{:?}", keymap);
}
