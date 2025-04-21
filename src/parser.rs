use clap::Parser;
use shift_schedule_generator::Shift;

#[derive(clap::Parser)]
struct Cli {
    names: Vec<String>,
    #[arg(short, long)]
    days: usize,
}

pub fn parse_args() -> (Vec<String>, Vec<Vec<Shift>>) {
    let day: Vec<Shift> = vec![
        Shift {
            name: String::from("breakfast preparing"),
            people: 3,
        },
        Shift {
            name: String::from("washing dishes"),
            people: 2,
        },
        Shift {
            name: String::from("cook lunch"),
            people: 3,
        },
        Shift {
            name: String::from("washing dishes"),
            people: 2,
        },
        Shift {
            name: String::from("dinner"),
            people: 3,
        },
        Shift {
            name: String::from("washing dishes"),
            people: 2,
        },
    ];
    let start = inquire::DateSelect::new("Pick a start date").prompt();
    let end = inquire::DateSelect::new("Pick a end date").prompt();
    let days;
    match (start, end) {
        (Ok(start), Ok(end)) => days = (end-start).num_days() + 1,
        default => days = 0,
    }

    let args = Cli::parse();
    
    (args.names, vec![day; days as usize])
}
