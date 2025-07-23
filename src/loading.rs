use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub struct LoadingSpinner {
    spinner: ProgressBar,
}

impl LoadingSpinner {
    pub fn new(message: &str) -> Self {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.set_message(message.to_string());
        spinner.enable_steady_tick(Duration::from_millis(80));
        Self { spinner }
    }

    pub fn update_message(&self, message: &str) {
        self.spinner.set_message(message.to_string());
    }

    pub fn finish_with_success(&self, message: &str) {
        self.spinner.finish_with_message(format!("✅ {}", message));
    }

    pub fn finish_with_error(&self, message: &str) {
        self.spinner.finish_with_message(format!("❌ {}", message));
    }

    pub fn finish_and_clear(&self) {
        self.spinner.finish_and_clear();
    }
}

impl Drop for LoadingSpinner {
    fn drop(&mut self) {
        self.spinner.finish_and_clear();
    }
}
