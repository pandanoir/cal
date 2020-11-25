mod calendar;
use crate::calendar::{create_annual_calendar, get_calendar};
use chrono::{Datelike, Local, TimeZone};
use getopts::Options;
use std::env;

fn get_chunked_string(source: &Vec<Vec<String>>, row_size: u32) -> Vec<String> {
  let mut first_of_row = source.iter();
  let mut result: Vec<String> = Vec::new();
  for _ in 0..row_size {
    let row_items = first_of_row.clone().step_by(row_size as usize);
    let max_row_length = row_items.clone().map(|list| list.len()).max().unwrap_or(0);

    let mut res: Vec<String> = vec![String::new(); max_row_length];

    for elem in row_items {
      for i in 0..max_row_length {
        res[i] += &format!("{} ", elem.get(i).unwrap_or(&"   ".repeat(7)));
      }
    }
    result.append(&mut res);

    first_of_row.next();
  }
  result
}
fn create_option() -> getopts::Options {
  let mut opts = Options::new();
  opts.optflag("h", "help", "print this help menu");
  opts.optflag("a", "annual", "print annual calendar");
  opts.optflag(
    "",
    "monday",
    "print calendar with first day of the week as monday",
  );
  opts
}

fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} FILE [options]", program);
  print!("{}", opts.usage(&brief));
}
fn main() {
  let opts = create_option();

  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();
  let matches = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => {
      panic!(f.to_string())
    }
  };

  if matches.opt_present("help") {
    print_usage(&program, opts);
    return;
  }

  let uses_monday_as_first_day = matches.opt_present("monday");
  let today = Local::today();
  let row_size = 5;

  let (year, month) = match matches.free.len() {
    0 => (today.year(), today.month()),
    1 => (today.year(), matches.free[0].parse().unwrap()),
    _ => (
      matches.free[1].parse().unwrap(),
      matches.free[0].parse().unwrap(),
    ),
  };

  if matches.opt_present("annual") {
    let calendar = create_annual_calendar(year, today, uses_monday_as_first_day);
    for elem in get_chunked_string(&calendar, row_size) {
      println!("{}", elem);
    }
  } else {
    let calendar = get_calendar(Local.ymd(year, month, 1), today, uses_monday_as_first_day);
    for elem in calendar {
      println!("{}", elem);
    }
  }
}
