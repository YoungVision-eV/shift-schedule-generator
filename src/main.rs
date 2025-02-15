use shift_schedule_generator::*;

fn main() {
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
    print_schedule(schedule);
}
