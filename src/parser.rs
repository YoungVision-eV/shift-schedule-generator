use chrono::NaiveDate;
use clap::{Parser};
use inquire::{CustomType, DateSelect, MultiSelect, Text};
use std::{path::PathBuf, process::exit};

use crate::schedule_table::{ScheduleTable};

#[derive(clap::Parser)]
struct Cli {
    #[arg(short, long, value_name="YYYY-MM-DD")]
    first_date: Option<NaiveDate>,
    #[arg(short, long, value_name="YYYY-MM-DD")]
    last_date: Option<NaiveDate>,
    #[arg(long, value_name="FILE")]
    csv: Option<PathBuf>,
    #[arg(long, value_name="FILE")]
    md: Option<PathBuf>,
    #[arg(long, value_name="FILE")]
    html: Option<PathBuf>,
    names: Vec<String>,
}

pub struct ParsedArgs {
    first_date: NaiveDate,
    last_date: NaiveDate,
    pub csv_path: Option<PathBuf>,
    pub md_path: Option<PathBuf>,
    pub html_path: Option<PathBuf>,
    pub names: Vec<String>,
}

/**
 * Prompts the user which shifts to disable and then disables them
 */
fn prompt_disabled_shifts(schedule: &mut ScheduleTable) {
    let all_days: Vec<_> = schedule.iter_dates().collect();
    let ans = MultiSelect::new(
        "Select days one which you want to disable shifts: ",
        all_days.clone(),
    )
    .prompt();
    let selected_days = match ans {
        Ok(d) => d,
        Err(_) => return,
    };
    let shift_options: Vec<_> = selected_days
        .iter()
        .flat_map(|d| {
            schedule
                .shift_labels
                .iter()
                .map(move |s| format!("{}: {}", d, s))
        })
        .collect();
    let ans = MultiSelect::new("Select shifts to disable.", shift_options.clone()).prompt();
    let selected_shifts = match ans {
        Ok(s) => s,
        Err(_) => return,
    };
    for selected in selected_shifts {
        let pos = shift_options.iter().position(|s| *s == selected).unwrap();
        let day = selected_days[pos / schedule.shift_labels.len()];
        let index_day = all_days.iter().position(|d| *d == day).unwrap();
        schedule.disable_shift(index_day, pos % schedule.shift_labels.len());
    }
}

pub fn parse_args() -> (ScheduleTable, ParsedArgs) {
    let args = Cli::parse();
    let mut result = ParsedArgs {
        first_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        last_date: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
        csv_path: args.csv,
        md_path: args.md,
        html_path: args.html,
        names: Vec::new(),
    };
    if args.names.len() == 0 {
        eprintln!("Enter a list of Names seperated by <Enter>. Enter empty name or Press Ctrl+D to finish the list.");
        while let Ok(name) = Text::new("").prompt() {
            if name.len() == 0 {
                break;
            }
            result.names.push(name);
        }
    }
    else {
        result.names = args.names;
    }
    result.first_date = match args.first_date {
        Some(d) => d,
        None => DateSelect::new("Pick a start date")
            .prompt()
            .expect("Error: invalid first_day"),
    };
    result.last_date = match args.last_date {
        Some(d) => d,
        None => DateSelect::new("Pick a end date")
            .with_min_date(result.first_date)
            .prompt()
            .expect("Error, invalid last_day"),
    };
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
    let mut shift_labels: Vec<String> = Vec::new();
    // the number of peoples needed for each shift
    let mut shift_sizes: Vec<usize> = Vec::new();
    for _ in 0..n_shifts {
        let shift_label = Text::new("Title of the shift:").prompt();
        shift_labels.push(shift_label.expect("Error: invalid shift_title"));
        let n_people: Result<usize, inquire::InquireError> =
            CustomType::<usize>::new("How many persons?")
                .with_error_message("Please type a valid number")
                .prompt();
        shift_sizes.push(n_people.expect("Error: invalid n_people"));
    }
    let mut schedule = ScheduleTable::new(result.first_date, result.last_date, shift_labels, shift_sizes);
    prompt_disabled_shifts(&mut schedule);
    (schedule, result)
}
