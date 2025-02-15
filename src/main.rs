use shift_schedule_generator::*;
mod parser;

fn main() {
    let args= parser::parse_args();
    let schedule = fill_schedule(&args.1, &args.0);
    print_schedule(schedule);
}
