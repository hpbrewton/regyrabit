mod ticker;
mod scheduler;
use crate::ticker::Ticker;
use crate::scheduler::Scheduler;

fn main() {
    let mut rr = scheduler::new();
    for i in 0..10 {
        rr.schedule(ticker::SimpleTicker{
            count: 5
        })
    }
    let mut count = 0;
    while rr.tick() {
        count += 1;
    }
    println!("{}", count);
}
