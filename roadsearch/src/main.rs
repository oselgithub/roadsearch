extern crate getopts;
extern crate roadsearch;

use getopts::Options;
use roadsearch::map::RoadMap;
use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn print_usage(program: &str, opts: Options) {
    println!("{}", opts.usage(&format!("Usage: {} [options] <data-path> <start> <destination>", program)));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let mut opts = Options::new();
  opts.optflag("h", "help", "Show this usage message.");
  let matches = match opts.parse(&args[1..]) {
    Ok(m)  => { m },
    Err(e) => { panic!(e.to_string()) },
  };
  if matches.opt_present("h") {
    print_usage(&program, opts);
    return;
  }
  let data_path = Path::new(&args[1]);
  let mut file = File::open(data_path).unwrap();
  let mut file_content = String::new();
  file.read_to_string(&mut file_content).unwrap();
  let map = RoadMap::from_str(&file_content).unwrap();
  map.print_path(&args[2], &args[3]);
}
