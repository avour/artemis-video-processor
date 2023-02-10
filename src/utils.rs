use std::path::Path;

/// utils

pub fn get_number_of_reels(duration: i64, max_reels_duration: i64) -> i64 {
    let mut number_of_reels = duration / max_reels_duration;

    // if there is extra reels > 10 seconds
    if duration % 30 > 10 {
        number_of_reels += 1;
    }
    number_of_reels
}

pub fn convert_timestamp(seconds: i64) -> String {
    let mut sec = seconds;
    let mut hours = 0;
    let mut minutes = 0;
    if sec >= 3600 {
        hours = sec / 3600;
        sec = sec % 3600;
    }
    if sec >= 60 {
        minutes = sec / 60;
        sec = sec % 60;
    }
    format!("{:02}:{:02}:{:02}", hours, minutes, sec)
}

pub fn get_video_duration<P: AsRef<Path>>(path: &P) -> i64 {
    let context = ffmpeg::format::input(path).unwrap();
    context.duration() / 1000000 // converts to seconds
}
