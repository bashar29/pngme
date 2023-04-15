use chrono::{DateTime, Local};

fn main() {
    let now: DateTime<Local> = Local::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%H%M%S%d%m%Y"));
}
