use chrono::NaiveDate;

#[derive(Clone)]
struct Shift {
    people: Vec<String>,
}

struct Day {
    date: NaiveDate,
    shifts: Vec<Option<Shift>>,
}

struct ScheduleTable {
    data: Vec<Vec<Option<Shift>>>,
    shift_names: Vec<String>,
    start: NaiveDate,
    end: NaiveDate,
}

impl ScheduleTable {
    fn new(start: NaiveDate, end: NaiveDate, shift_names: Vec<String>) -> Self {
        Self {
            data: vec![vec![None; shift_names.len()]; (end - start).num_days() as usize],
            shift_names: shift_names,
            start: start,
            end: end,
        }
    }
    pub fn get_days(&self, index: usize) -> Vec<Day> {
        self.data
            .iter()
            .enumerate()
            .map(|(i, column)| Day {
                date: self
                    .start
                    .checked_add_days(chrono::Days::new(i as u64))
                    .expect("Date out of range."),
                shifts: column.clone(),
            })
            .collect()
    }
} 
