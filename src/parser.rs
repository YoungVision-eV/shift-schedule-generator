use clap::{error, Parser};
use inquire::{CustomType, DateSelect, MultiSelect, Text};
use std::{collections::HashMap, process::exit};

use crate::schedule_table::{ScheduleTable, Shift};

#[derive(clap::Parser)]
struct Cli {
    names: Vec<String>,
    #[arg(short, long)]
    days: Option<usize>,
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
            schedule.shift_labels
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

pub fn parse_args() -> ScheduleTable {
    let first_day = DateSelect::new("Pick a start date").prompt().expect("Error: invalid first_day");
    let last_day = DateSelect::new("Pick a end date").with_min_date(first_day).prompt().expect("Error, invalid last_day");
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
        let n_people: Result<usize, inquire::InquireError> = CustomType::<usize>::new("How many persons?")
            .with_error_message("Please type a valid number")
            .prompt();
        shift_sizes.push(n_people.expect("Error: invalid n_people"));
    }
    let mut schedule = ScheduleTable::new(
        first_day,
        last_day,
        shift_labels,
        shift_sizes,
    );
    prompt_disabled_shifts(&mut schedule);

    schedule
}
