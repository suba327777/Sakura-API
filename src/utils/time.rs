use chrono::{Local, NaiveDateTime};

pub fn create_time() -> NaiveDateTime {
    let local_now = Local::now();
    local_now.naive_local()
}
