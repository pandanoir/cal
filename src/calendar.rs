use chrono::{Date, Datelike, Local, TimeZone};

type MonthlyCalendar = Vec<String>;
type AnnualCalendar = Vec<MonthlyCalendar>;

fn last_date(year: i32, month: u32) -> Date<Local> {
  Local
    .ymd_opt(year, month + 1, 1)
    .earliest()
    .unwrap_or(Local.ymd(year + 1, 1, 1))
    .pred()
}
fn red(s: String) -> String {
  format!("\x1b[31m{}\x1b[0m", s)
}

pub fn get_calendar(first_date: Date<Local>, today: Date<Local>) -> MonthlyCalendar {
  let (year, month) = (first_date.year(), first_date.month());
  let mut res: MonthlyCalendar = Vec::new();
  let header = format!("{:-^21}", format!(" {}-{} ", year, month));
  res.push(header);

  let mut i = first_date.weekday().num_days_from_sunday();
  let mut line = "   ".repeat(i as usize);

  for date in first_date.day()..last_date(year, month).day() + 1 {
    let cell_string = if today == Local.ymd(year, month, date) {
      red(format!("{:2}", date))
    } else {
      format!("{:2}", date)
    };
    line += &cell_string;
    line += " ";
    i += 1;
    if i == 7 {
      res.push(line);
      line = String::new();
      i = 0;
    }
  }
  line += &"   ".repeat(6 - last_date(year, month).weekday().num_days_from_sunday() as usize);
  if line.len() > 0 {
    res.push(line)
  }
  res
}
pub fn create_annual_calendar(year: i32, today: Date<Local>) -> AnnualCalendar {
  (1..13)
    .map(|month| get_calendar(Local.ymd(year, month, 1), today))
    .collect()
}
