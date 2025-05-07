mod process;
mod parser;
mod schedule_table;
pub use crate::{schedule_table::ScheduleTable, process::*};

fn main() {
    let schedule: ScheduleTable = parser::parse_args();
    // let schedule = fill_schedule(&args.1, &args.0);
    print_schedule_md(&schedule);
}
