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
    let args = Cli::parse();
    println!("{:?}", args.names);
    println!("{:?}", args.days);
    (args.names, vec![day; args.days])
}
