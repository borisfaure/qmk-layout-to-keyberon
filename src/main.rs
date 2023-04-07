use clap::{value_parser, Arg, ArgAction, Command};
use keyberon_layout_serde::keyberon::Layers;
use keyberon_layout_serde::qmk::QmkKeyMap;
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
        .arg(
            Arg::new("cols")
                .value_name("COLS")
                .short('c')
                .long("cols")
                .num_args(1)
                .required(true)
                .help("number of cols in the layout")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("rows")
                .value_name("ROWS")
                .short('r')
                .long("rows")
                .num_args(1)
                .required(true)
                .help("number of rows in the layout")
                .value_parser(value_parser!(u8)),
        )
        .arg(
            Arg::new("is_split")
                .value_name("IS_SPLIT")
                .short('s')
                .long("is-split")
                .action(ArgAction::SetTrue)
                .num_args(0)
                .required(false)
                .help("whether the layout is split"),
        )
        .arg(
            Arg::new("ignore_errors")
                .value_name("IGNORE_ERRORS")
                .short('i')
                .long("ignore-errors")
                .action(ArgAction::SetTrue)
                .num_args(0)
                .required(false)
                .help("whether to ignore conversion errors"),
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
    let key_map_res = QmkKeyMap::from_json_str(&s);
    if let Err(ref err) = key_map_res {
        println!("{:?}", err);
    }
    let key_map = key_map_res.unwrap();

    let cols = matches.get_one::<u8>("cols").unwrap();
    let rows = matches.get_one::<u8>("rows").unwrap();
    let is_split = matches.get_flag("is_split");
    let ignore_errors = matches.get_flag("ignore_errors");

    let layers_res = Layers::try_from(
        key_map,
        *cols as usize,
        *rows as usize,
        is_split,
        ignore_errors,
    );
    if let Err(ref err) = layers_res {
        println!("{:?}", err);
    }
    let layers = layers_res.unwrap();
    println!("{:?}", layers);
}
