use std::time::{Instant, Duration};
use piece_of_pi::calc_pi;

fn main() {
    let i = 250;

    // let now = Instant::now();
    // println!("Leib({})::Pi = {}",i,  calc_pi::leibniz(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // let now = Instant::now();
    // println!("Chud({})::Pi = {}",i,  calc_pi::chudnovsky(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // let now = Instant::now();
    // println!("Chud({})::Pi = {}",i,  calc_pi::BIGDchudnovsky(i)); 
    // let elapsed_time = now.elapsed();
    // println!("It took {} to run", format_time(elapsed_time));

    // println!("BigFloat::Pi = {}", num_bigfloat::PI);


    let minute_nhalf = Duration::from_secs(90);
    let minute = Duration::from_secs(60);
    let second = Duration::from_secs(1);
    let nanosec = Duration::from_nanos(1);
    let microsec = Duration::from_micros(1);
    let millisec = Duration::from_millis(1);

    let elapsed_time = millisec;
    println!("It took {} to run", format_time(elapsed_time));

    let elapsed_time = microsec; 
    println!("It took {} to run", format_time(elapsed_time));

    let elapsed_time = nanosec;
    println!("It took {} to run", format_time(elapsed_time));

    let elapsed_time = second;
    println!("It took {} to run", format_time(elapsed_time));

    let elapsed_time = minute;
    println!("It took {} to run", format_time(elapsed_time));

    let elapsed_time = minute_nhalf;
    println!("It took {} to run", format_time(elapsed_time));

} 

fn format_time(duration: Duration) -> String {
    // TODO utilize modulo and shit to make this shit run off "4 days 6 hours 3 minutes and 20 seconds"
    // think this should be libraried and each level over nano needs run off in order to be precise
    const DAY_AS_NANO: u128 = OUR_AS_NANO * 24; // 1 day
    const OUR_AS_NANO: u128 = MIN_AS_NANO * 60; // 1 hour
    const MIN_AS_NANO: u128 = SEC_AS_NANO * 60; // 1 minute
    const SEC_AS_NANO: u128 = 1000000000; // 1 second
    const MIL_AS_NANO: u128 = 1000000; // 1 millisecond
    const MIC_AS_NANO: u128 = 1000; // 1 microsecond
    let pretty_time: String = match duration.as_nanos() {
        nano if nano <  MIC_AS_NANO => format!("{} nanoseconds", duration.as_nanos()),
        mcro if mcro <  MIL_AS_NANO => format!("{} microseconds", duration.as_micros()),
        mili if mili <  SEC_AS_NANO => format!("{} milliseconds", duration.as_millis()),
        secs if secs <  MIN_AS_NANO => format!("{} seconds", duration.as_secs()),
        mins if (mins >= MIN_AS_NANO) &&  (duration.as_secs()%60 == 0) => format!("{} minutes", duration.as_secs()/60),
        mins if (mins >= MIN_AS_NANO) && !(duration.as_secs()%60 == 0) => format!("{} minutes {} seconds", duration.as_secs()/60, duration.as_secs()%60),
        _ => String::from("SOMETHING HORRIFIC HAS JUST HAPPENED!")
        
    };
    pretty_time
}
