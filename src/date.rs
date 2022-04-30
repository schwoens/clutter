
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u16) -> Self {
        Self{day, month, year}
    }
}