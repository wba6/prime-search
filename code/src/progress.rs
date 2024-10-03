// src/progress.rs

use indicatif::{ProgressBar, ProgressStyle};

pub fn create_progress_bar(length: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(length);
    let style = ProgressStyle::default_bar()
        .template("{msg} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .unwrap()
        .progress_chars("##-");
    pb.set_style(style);
    pb.set_message(message.to_string()); // Convert &str to String
    pb
}
