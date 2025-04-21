use std::process::exit;

use clap::{error, Parser};
use shift_schedule_generator::Shift;

#[derive(clap::Parser)]
struct Cli {
    names: Vec<String>,
    #[arg(short, long)]
    days: Option<usize>,
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
    let days = match (start, end) {
        (Ok(start), Ok(end)) => (end - start).num_days() + 1,
        default => 0,
    };
    if days <= 0 {
        eprintln!("Invalid Dates");
        exit(1);
    }
    let prompt_n_shifts = inquire::CustomType::<usize>::new("How many shifts per day?")
        .with_error_message("Please type a valid number")
        .prompt();
    let n_shifts = match prompt_n_shifts {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error");
            exit(1);
        }
    };
    for _ in 0..n_shifts {
        let shift_title = inquire::Text::new("Title of the shift:").prompt();
        let n_people = inquire::CustomType::<usize>::new("How many persons?")
            .with_error_message("Please type a valid number")
            .prompt();
    }

    let args = Cli::parse();

    (args.names, vec![day; days as usize])
}
