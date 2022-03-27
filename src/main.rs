use clap::Parser;
use log::{info, warn};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Parser)]
struct Cli {
    size: usize,
    quantity: usize,
    dir_name: String,
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    let dir_name = args.dir_name;

    match fs::create_dir(&dir_name) {
        Err(why) => warn!("{} directory is {:?}", &dir_name, why.kind()),
        Ok(_) => {}
    }

    let b = b"0";
    for _ in 0..args.quantity {
        let uuid = Uuid::new_v4().to_simple().to_string();
        let path_str = dir_name.to_string() + &"/" + &uuid;
        let path = Path::new(&path_str);
        let mut writer = BufWriter::new(File::create(path).unwrap());
        for _ in 0..args.size {
            writer.write(b).unwrap();
        }
        info!("{} created.", &uuid)
    }
}
