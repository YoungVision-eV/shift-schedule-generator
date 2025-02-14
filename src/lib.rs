#[derive(Debug, Clone, Default)]
pub struct Shift {
    pub name: String,
    pub people: usize,
}

pub fn fill_schedule(
    schedule: &Vec<&Vec<Shift>>,
    people: Vec<String>,
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
        let schedule = fill_schedule(&vec![&first_day, &day, &day, &last_day], names);
    }
}
