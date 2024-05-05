use std::time::{SystemTime, UNIX_EPOCH};

const SUFFIX: [&str; 9] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

const UNIT: f64 = 1024.0;

/// Converts bytes to human-readable values
pub fn human_bytes<T: Into<f64>>(bytes: T) -> String {
    let size = bytes.into();

    if size <= 0.0 {
        return "0 B".to_string();
    }

    let base = size.log10() / UNIT.log10();

    let result = format!("{:.1}", UNIT.powf(base - base.floor()),)
        .trim_end_matches(".0")
        .to_owned();

    // Add suffix
    [&result, SUFFIX[base.floor() as usize]].join(" ")
}

pub fn shorten_string(text: String, max_len: usize) -> String {
    let len = text.len();
    if len > max_len {
        let prefix = &text[0..5];

        let suffix = &text[len - 5..len];

        format!("{}...{}", prefix, suffix)
    } else {
        text
    }
}

pub fn get_unix_timestamp() -> u64 {
    let time = SystemTime::now();
    time.duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
