use chrono::{Duration, NaiveTime, Timelike};

pub fn sec_to_h_m_s_str(secs: i64) -> String {
    match sec_to_naive_time(secs) {
        None => "".to_string(),
        Some(t) => {
            if t.hour() > 0 {
                t.format("%Hh %Mm %Ss").to_string()
            } else {
                t.format("%Mm %Ss").to_string()
            }
        }
    }
}

pub fn sec_to_h_m_str(secs: i64) -> String {
    match sec_to_naive_time(secs) {
        None => "".to_string(),
        Some(t) => {
            if t.hour() > 0 {
                t.format("%Hh %Mm").to_string()
            } else {
                t.format("%Mm").to_string()
            }
        }
    }
}

fn sec_to_naive_time(secs: i64) -> Option<NaiveTime> {
    let duration = Duration::seconds(secs);

    let hours = duration.num_hours();
    let minutes = (duration.num_minutes() % 60) as u32;
    let seconds = (duration.num_seconds() % 60) as u32;

    let hours = hours as u32 % 24;
    let minutes = minutes % 60;
    let seconds = seconds % 60;

    NaiveTime::from_hms_opt(hours, minutes, seconds)
}