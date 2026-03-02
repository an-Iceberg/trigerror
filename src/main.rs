use clap::Parser;
use trigerror::cli::CLI;
use ini::ini;

fn main()
{
  // let cli = CLI::parse();

  // println!("interface(s): {:?}", cli.interfaces);

  let config = ini!("trigerror.ini");
  dbg!(config.clone());

  let interfaces = config["default"]["interfaces"]
    .clone()
    .unwrap()
    .split(",")
    .map(|interface| interface.trim().to_string())
    .collect::<Vec<String>>();
  dbg!(interfaces);

  let filters = config["default"]["filters"]
    .clone()
    .unwrap() // NOTE: since this is an option at this point we'd handle the None case.
    .split(",")
    .map(|filter| filter.trim().to_string())
    .collect::<Vec<String>>();
  dbg!(filters);

  let count_before = config["default"]["count_before"]
    .clone()
    .unwrap()
    .parse::<u32>()
    .unwrap();
  dbg!(count_before);

  let count_after = config["default"]["count_after"]
    .clone()
    .unwrap()
    .parse::<u32>()
    .unwrap();
  dbg!(count_after);

  let time_before = config["default"]["time_before"]
    .clone()
    .unwrap()
    .parse::<f32>()
    .unwrap();
  dbg!(time_before);

  let time_after = config["default"]["time_after"]
    .clone()
    .unwrap()
    .parse::<f32>()
    .unwrap();
  dbg!(time_after);

  let retrigger = config["default"]["retrigger"]
    .clone()
    .unwrap()
    .parse::<bool>()
    .unwrap();
  dbg!(retrigger);
}
