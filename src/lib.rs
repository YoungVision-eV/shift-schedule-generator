#[derive(Debug, Clone, Default)]
pub struct Shift {
    pub name: String,
    pub people: usize,
}

pub fn fill_schedule(
    schedule: &Vec<&Vec<Shift>>,
    people: &Vec<String>,
) -> Vec<Vec<(String, Vec<String>)>> {
    let mut i_people = 0;
    let mut result = Vec::new();
    for day in schedule {
        let mut result_day = Vec::new();
        for shift in *day {
            let mut result_shift = (shift.name.clone(), Vec::new());
            for _ in 0..shift.people {
                result_shift.1.push(people[i_people].clone()); //TODO: Try to remove this clone, bc it may be unecessary and to learn about rust lifetime
                i_people += 1;
                if i_people >= people.len() {
                    i_people -= people.len();
                }
            }
            result_day.push(result_shift);
        }
        result.push(result_day);
    }
    result
}

pub fn print_schedule(schedule: Vec<Vec<(String, Vec<String>)>>) {
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
        let first_day: Vec<Shift> = vec![
            Shift {
                name: String::from("dinner"),
                people: 3,
            },
            Shift {
                name: String::from("washing dishes"),
                people: 2,
            },
        ];
        let last_day: Vec<Shift> = vec![
            Shift {
                name: String::from("breakfast preparing"),
                people: 2,
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
        ];
        let schedule = fill_schedule(&vec![&first_day, &day, &day, &last_day], &names);
        let flat_schedule = schedule.concat();
        assert!(!double_asignments(flat_schedule.clone()));
        assert!(fair_distribution(flat_schedule, names));
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
