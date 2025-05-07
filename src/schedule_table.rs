use chrono::{NaiveDate, Duration};
use std::collections::{HashMap};

#[derive(Clone)]
pub enum Shift {
    Unkown,
    Disabled,
    Empty(usize),
    Filled(Vec<String>),
}

struct Day {
    date: NaiveDate,
    shifts: Vec<Shift>,
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
    pub fn new(first_day: NaiveDate, last_day: NaiveDate, shift_labels: Vec<String>, shift_sizes: Vec<usize>) -> Self {
        //TODO: maybe use one Vec of tuples instead of two Vec to ensure same length
        if shift_labels.len() != shift_sizes.len() {
            panic!("shift_lables and shift_sizes must have the same length");
        }
        let shifts = shift_sizes.iter().map(|s| Shift::Empty(*s)).collect();
        Self {
            data: vec![shifts; (last_day - first_day).num_days() as usize],
            shift_labels,
            first_day,
            last_day,
        }
    }
    //TODO: rethink this method
    pub fn get_days(&self, index: usize) -> Vec<Day> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, column)| Day {
                date: self
                    .first_day
                    .checked_add_days(chrono::Days::new(i as u64))
                    .expect("Date out of range."),
                shifts: column.clone(),
            })
            .collect()
    }

    pub fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
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
}
