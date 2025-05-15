use crate::schedule_table::{ScheduleTable};
use chrono::Datelike;

#[derive(Debug, Clone, Default)]
pub struct DeprecatedShift {
    pub name: String,
    pub people: usize,
}

pub fn print_schedule(schedule: &Vec<Vec<(String, Vec<String>)>>) {
    for (i, day) in schedule.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!("Tag {}", i);
        for shift in day {
            println!("{}: {:?}", shift.0, shift.1)
        }
    }
}

pub fn print_schedule_md(schedule: &ScheduleTable) {
    print!("| ");
    for d in schedule.iter_dates() {
        print!("| {} {}.{}. ", d.weekday(), d.day(), d.month());
    }
    println!("|");
    print!("| --------: ");
    for _ in schedule.iter_dates() {
        print!("| :--------: ");
    }
    println!("|");
    for i in 0..schedule.shift_labels.len() {
        print!("| {} ", schedule.shift_labels[i]);
        for j in 0..schedule.iter_dates().count() {
            print!("| {} ", schedule.get_shift(j, i));
        }
        println!("|");
    }
}

pub fn print_schedule_csv<W: std::io::Write>(schedule: &ScheduleTable, writer: &mut W) -> std::io::Result<()> {
    write!(writer, "Shift, ")?;
    for d in schedule.iter_dates() {
        write!(writer, "{}, ", d)?;
    }
    writeln!(writer)?;
    for i in 0..schedule.shift_labels.len() {
        write!(writer, "\"{}\", ", schedule.shift_labels[i])?;
        for j in 0..schedule.iter_dates().count() {
            write!(writer, "\"{}\", ", schedule.get_shift(j, i))?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

pub fn print_schedule_html(schedule: &ScheduleTable) {
    println!(
        "<style>
table, th, td {{
  border:1px solid black;
  padding: 0.25rem;
  text-align: center;
}}
</style>"
    );
    println!("<table>");
    println!("  <tr>");
    println!("    <th></th>");
    for d in schedule.iter_dates() {
        println!("    <th>{} {}.{}.</th>", d.weekday(), d.day(), d.month());
    }
    println!("  </tr>");
    for (i, label) in schedule.shift_labels.iter().enumerate() {
        println!("  <tr>");
        println!("    <td><strong>{}</strong></td>", label);
        for j in 0..schedule.iter_dates().count() {
            println!("    <td>{}</td>", schedule.get_shift(j, i));
        }
        println!("  </tr>");
    }

    println!("</table>");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn basic_test() {
        let names = vec![
            String::from("Alice"),
            String::from("Bob"),
            String::from("Charlie"),
            String::from("Dave"),
            String::from("Eve"),
            String::from("Frank"),
            String::from("Grace"),
            String::from("Heidi"),
        ];

        let day: Vec<DeprecatedShift> = vec![
            DeprecatedShift {
                name: String::from("breakfast preparing"),
                people: 3,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
            DeprecatedShift {
                name: String::from("cook lunch"),
                people: 3,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
            DeprecatedShift {
                name: String::from("dinner"),
                people: 3,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
        ];
        let first_day: Vec<DeprecatedShift> = vec![
            DeprecatedShift {
                name: String::from("dinner"),
                people: 3,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
        ];
        let last_day: Vec<DeprecatedShift> = vec![
            DeprecatedShift {
                name: String::from("breakfast preparing"),
                people: 2,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
            DeprecatedShift {
                name: String::from("cook lunch"),
                people: 3,
            },
            DeprecatedShift {
                name: String::from("washing dishes"),
                people: 2,
            },
        ];
        // let schedule = fill_schedule(&vec![first_day, day.clone(), day.clone(), last_day], &names);
        // let flat_schedule = schedule.concat();
        // assert!(!double_asignments(flat_schedule.clone()));
        // assert!(fair_distribution(flat_schedule, names));
    }

    fn double_asignments(schedule: Vec<(String, Vec<String>)>) -> bool {
        for shift in schedule {
            if shift.1.len() > HashSet::<String>::from_iter(shift.1).len() {
                return true;
            }
        }
        false
    }

    fn fair_distribution(schedule: Vec<(String, Vec<String>)>, names: Vec<String>) -> bool {
        let list = schedule
            .iter()
            .map(|s| s.1.clone())
            .collect::<Vec<Vec<String>>>()
            .concat();
        let mut shift_quantities: Vec<usize> = names
            .iter()
            .map(|name| list.iter().filter(|l| *l == name).count())
            .collect();
        shift_quantities.sort();
        *shift_quantities.first().unwrap() >= *shift_quantities.last().unwrap() - 1
    }
}
