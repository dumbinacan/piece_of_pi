use piece_of_pi::calc_pi;
use time_formatter::format_time::{TimeFormatter, TimeUnit};

use std::time::Instant;

fn main() {
    let time_formatter = TimeFormatter::from(TimeUnit::Day, TimeUnit::Nanosecond);

    for i in [250] /* [250, 500, 750, 1000] */ {
        let now = Instant::now();
        calc_pi::chudnovsky(i);
        let elapsed_time = now.elapsed();
        println!("It took {} to run Chud({})", time_formatter.format(elapsed_time), i);
    }
}
