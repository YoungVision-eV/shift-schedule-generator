use std::process::exit;

use clap::{error, Parser};
use inquire::{CustomType, DateSelect, MultiSelect, Text};
use shift_schedule_generator::Shift;

#[derive(clap::Parser)]
struct Cli {
    names: Vec<String>,
    #[arg(short, long)]
    days: Option<usize>,
}

fn prompt_disabled_shifts(n_days: usize, n_shifts: usize) {
    let all_days = (1..n_days).collect();
    let ans = MultiSelect::new("Select days one which you want to disable shifts: ", all_days).prompt();
    let selected_days = match ans {
        Ok(d) => d,
        Err(_) => todo!("unhandled error"),
    };
    let shifts_per_day: Vec<_> = (1..n_shifts).collect();
    let shift_choices: Vec<_> = selected_days.iter().flat_map(|d| {
        shifts_per_day.iter().map(move |s| format!("Day {}: Shift {}", d, s))
    }).collect();
    let ans = MultiSelect::new("Select shifts to disable.", shift_choices).prompt();
    let selected_shifts = match ans {
        Ok(s) => s,
        Err(_) => todo!("unhandled error"),
    };
    println!("{:?}", selected_shifts);
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
    let start = DateSelect::new("Pick a start date").prompt();
    let end = DateSelect::new("Pick a end date").prompt();
    let days = match (start, end) {
        (Ok(start), Ok(end)) => (end - start).num_days() + 1,
        default => 0,
    };
    if days <= 0 {
        eprintln!("Invalid Dates");
        exit(1);
    }
    let prompt_n_shifts = CustomType::<usize>::new("How many shifts per day?")
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
        let shift_title = Text::new("Title of the shift:").prompt();
        let n_people = CustomType::<usize>::new("How many persons?")
            .with_error_message("Please type a valid number")
            .prompt();
    }
    prompt_disabled_shifts(days as usize, n_shifts);

    let args = Cli::parse();

    (args.names, vec![day; days as usize])
}
