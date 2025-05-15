use chrono::{Duration, NaiveDate};
use std::{fmt, usize};

#[derive(Clone)]
pub enum Shift {
    Unkown,
    Disabled,
    Empty(usize),
    Filled(Vec<String>),
}

impl fmt::Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shift::Unkown => write!(f, "?"),
            Shift::Disabled => write!(f, " / "),
            Shift::Empty(n) => write!(f, "{} empty slots", n),
            Shift::Filled(people) => write!(f, "{}", people.join(", ")),
        }
    }
}

pub struct ScheduleTable {
    /**
    The 2D Vec holding the data for the table i.e the Shifts with the names
    *  The outer Vec is for the days and the inner Vec for the shifts in a day,
       so you can acces a specific shift like this `data[index_day][index_shift]`.
    *  The table should always be rectangular i.e every Vec inside the outer Vec has the same lenght.
    */
    data: Vec<Vec<Shift>>,
    /// The shift names Vec should always have the same length as the inner Vec from data
    pub shift_labels: Vec<String>,
    first_day: NaiveDate,
    last_day: NaiveDate,
}

impl ScheduleTable {
    /**
     * creates a Table with the correct size
     * all shifts are set to [Unkown](Shift::Unkown)
     */
    pub fn new(
        first_day: NaiveDate,
        last_day: NaiveDate,
        shift_labels: Vec<String>,
        shift_sizes: Vec<usize>,
    ) -> Self {
        //TODO: maybe use one Vec of tuples instead of two Vec to ensure same length
        if shift_labels.len() != shift_sizes.len() {
            panic!("shift_lables and shift_sizes must have the same length");
        }
        let shifts = shift_sizes.iter().map(|s| Shift::Empty(*s)).collect();
        Self {
            data: vec![shifts; ((last_day - first_day).num_days() + 1) as usize],
            shift_labels,
            first_day,
            last_day,
        }
    }

    pub fn iter_dates(&self) -> impl Iterator<Item = NaiveDate> {
        let mut current = self.first_day;
        let end = self.last_day;

        std::iter::from_fn(move || {
            if current > end {
                None
            } else {
                let today = current;
                current = current.checked_add_signed(Duration::days(1))?;
                Some(today)
            }
        })
    }

    pub fn get_shift(&self, index_day: usize, index_shift: usize) -> Shift {
        self.data
            .get(index_day)
            .and_then(|day| day.get(index_shift))
            .cloned()
            .unwrap_or(Shift::Unkown)
    }

    pub fn disable_shift(&mut self, index_day: usize, index_shift: usize) {
        if let Some(day) = self.data.get_mut(index_day) {
            if let Some(shift) = day.get_mut(index_shift) {
                *shift = Shift::Disabled;
            }
        }
    }

    pub fn fill_schedule(&mut self, people: &Vec<String>) {
        let mut i_people = 0;
        // self.data = self.data.iter().map(|day|)
        for day in &mut self.data {
            for shift in day {
                if let Shift::Empty(people_count) = shift {
                    let mut asigned_people: Vec<String> = Vec::new();
                    for _ in 0..*people_count {
                        asigned_people.push(people[i_people].clone()); //TODO: Try to remove this clone, bc it may be unecessary and to learn about rust lifetime
                        i_people = (i_people + 1) % people.len();
                    }
                    *shift = Shift::Filled(asigned_people);
                }
            }
        }
    }
}
