use crate::ticker;
use std::collections::VecDeque;

pub trait Scheduler {
    fn schedule(&mut self, ticker: ticker::Ticker);
}

pub struct RoundRobin {
    queue: VecDeque<Box<ticker::Ticker>>
}

pub fn new() -> RoundRobin {
    return RoundRobin {
        queue: VecDeque::new()
    }
}

impl Scheduler for RoundRobin {
    fn schedule(&mut self, &mut ticker: ticker::Ticker) {
        self.queue.push_back(ticker);
    }
}

impl ticker::Ticker for RoundRobin {
    fn tick(&mut self) -> bool {
        let opt_box_ticker = self.queue.pop_front();
        if opt_box_ticker.is_some() {
            let &mut ticker = Box::into_raw(opt_box_ticker.unwrap());
            if ticker.ticker() {
                self.schedule(ticker);
            }
            return true;
        }
        return false;
    }
}