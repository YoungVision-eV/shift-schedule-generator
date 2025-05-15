mod parser;
mod process;
mod schedule_table;
pub use crate::{process::*, schedule_table::ScheduleTable};

fn main() {
    let (mut schedule, people) = parser::parse_args();
    schedule.fill_schedule(&people);
    print_schedule_csv(&schedule);
}
