use super::ProgressReporter;
use crate::game::screens::LoadingScreen;
use crate::Result;

pub struct CenteredProgressReporter {
    display: LoadingScreen,
}

impl CenteredProgressReporter {
    pub fn new() -> Result<Self> {
        let display = LoadingScreen::new()?;
        display.show_initial()?;
        Ok(Self { display })
    }

    pub fn finish(&self) -> Result<()> {
        self.display.show_completion()
    }
}

impl ProgressReporter for CenteredProgressReporter {
    fn set_phase(&self, phase: String) {
        let _ = self.display.update_phase(&phase);
    }

    fn set_progress(&self, _progress: f64) {
        // Progress is handled by set_file_counts
    }

    fn set_current_file(&self, _file: Option<String>) {
        // Individual file updates not shown in centered display
    }

    fn set_file_counts(&self, processed: usize, total: usize) {
        if total > 0 {
            let progress = processed as f64 / total as f64;
            let _ = self.display.update_progress(progress, processed, total);
        }
    }

    fn update_spinner(&self) {
        self.display.update_spinner();
    }

    fn finish(&self) -> crate::Result<()> {
        self.display.show_completion()
    }
}
