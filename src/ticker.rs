pub trait Ticker {
    fn tick(&mut self) -> bool;
}

#[derive(Debug)]
pub struct SimpleTicker {
    pub count: u8
}

impl Ticker for SimpleTicker {
    fn tick(&mut self) -> bool {
        if (*self).count > 0 {
            (*self).count -= 1;
            return true;
        } else {
            return false;
        }
    }
}