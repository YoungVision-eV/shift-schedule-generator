mod parser;
mod process;
mod schedule_table;
use std::{fs::File, io::Write};

pub use crate::{process::*, schedule_table::ScheduleTable};

fn main() {
    let (mut schedule, args) = parser::parse_args();
    schedule.fill_schedule(&args.names);
    if let Some(path) = args.csv_path {
        match File::create(path) {
            Ok(mut file) => {
                print_schedule_csv(&schedule, &mut file).inspect_err(|e| eprintln!("{}", e)).ok();
            }
            Err(e) => eprintln!("{}", e),
        };
    }
}
