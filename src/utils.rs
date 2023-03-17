// use std::path::Path;
use std::process::Command;

pub fn get_number_of_reels(duration: i64, max_reels_duration: i64) -> i64 {
    if duration < max_reels_duration {
        return 1;
    }

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

// pub fn get_video_duration<P: AsRef<Path>>(path: &P) -> i64 {
//     let context = ffmpeg::format::input(path).unwrap();
//     context.
//     context.duration() / 1000000 // converts to seconds
// }

pub fn get_video_duration(path: &String) -> i64 {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-show_entries")
        .arg("format=duration")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(path)
        .output()
        .expect("failed to execute process");

    let duration = String::from_utf8_lossy(&output.stdout);
    let duration: f64 = duration.trim().parse().unwrap();
    let duration = duration as i64;
    duration
}
