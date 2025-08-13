use chrono::TimeDelta;

fn fmt_hms(w: i64, d: i64, h: i64, m: i64, s: i64, ms: i64) -> String {
    let we = match w {
        0 => "".to_string(),
        0..7 => format!("0{w}:"),
        _ => format!("{w}:"),
    };
    let da = match d {
        0 => "".to_string(),
        _ => format!("{d}:"),
    };
    let hr = match h {
        0 => "".to_string(),
        0..10 => format!("0{h}:"),
        _ => format!("{h}:"),
    };
    let mn = match m {
        0 => "00:".to_string(),
        0..10 => format!("0{m}:"),
        _ => format!("{m}:"),
    };
    let sec = match s {
        0 => "00".to_string(),
        0..10 => format!("0{s}"),
        _ => format!("{s}"),
    };
    let millis = match ms {
        10..100 => format!("0{ms}"),
        0..10 => format!("00{ms}"),
        _ => format!("{ms}"),
    };

    format!("{we}{da}{hr}{mn}{sec}.{millis}")
}


pub fn format_timedelta(time_since: TimeDelta) -> String {
    let weeks = time_since.num_weeks();
    let days = time_since.num_days() % 7;
    let hrs = time_since.num_hours() % 24;
    let mins = time_since.num_minutes() % 60;
    let secs = time_since.num_seconds() % 60;
    let millis = time_since.num_milliseconds() % 1000;
    fmt_hms(weeks, days, hrs, mins, secs, millis)
}
