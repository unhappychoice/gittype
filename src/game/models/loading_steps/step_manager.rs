use super::{
    CacheCheckStep, CloningStep, DatabaseInitStep, ExecutionContext, ExtractingStep,
    FinalizingStep, GeneratingStep, ScanningStep, Step, StepResult,
};
use crate::game::screens::loading_screen::ProgressReporter;
use crate::Result;

pub struct StepManager {
    steps: Vec<Box<dyn Step>>,
}

impl Default for StepManager {
    fn default() -> Self {
        Self::new()
    }
}

impl StepManager {
    pub fn new() -> Self {
        Self {
            steps: vec![
                Box::new(DatabaseInitStep),
                Box::new(CloningStep),
                Box::new(CacheCheckStep),
                Box::new(ScanningStep),
                Box::new(ExtractingStep),
                Box::new(GeneratingStep),
                Box::new(FinalizingStep),
            ],
        }
    }

    pub fn get_step_by_name(&self, step_name: &str) -> Option<&dyn Step> {
        self.steps
            .iter()
            .find(|step| step.step_name() == step_name)
            .map(|s| s.as_ref())
    }

    pub fn get_step_by_number(&self, number: usize) -> Option<&dyn Step> {
        self.steps
            .iter()
            .find(|step| step.step_number() == number)
            .map(|s| s.as_ref())
    }

    pub fn get_all_steps(&self) -> &[Box<dyn Step>] {
        &self.steps
    }

    pub fn step_name_to_step_number(&self, step_name: &str) -> usize {
        self.get_step_by_name(step_name)
            .map(|step| step.step_number())
            .unwrap_or(0)
    }

    pub fn execute_pipeline(&self, context: &mut ExecutionContext) -> Result<()> {
        for step in &self.steps {
            // Skip step if it can be skipped
            if step.can_skip(context) {
                continue;
            }

            // Skip remaining steps if cache was used (except finalization)
            if context.cache_used && step.step_type() != super::StepType::Finalizing {
                continue;
            }

            // Set current step for progress reporting
            if let Some(screen) = context.loading_screen {
                screen.set_step(step.step_type());

                // Initialize progress for steps that support it
                if step.supports_progress() {
                    // Initialize with 0% progress
                    screen.set_file_counts(step.step_type(), 0, 1, None);
                }
            }

            // Execute step
            let step_result = step.execute(context)?;

            // Mark step as completed after successful execution
            if let Some(screen) = context.loading_screen {
                // For steps that support progress, ensure they show 100% completion
                if step.supports_progress() {
                    screen.set_file_counts(step.step_type(), 1, 1, None);
                    // Small delay to ensure the completion is visible before transitioning
                    std::thread::sleep(std::time::Duration::from_millis(200));
                } else {
                    // Even non-progress steps get a small delay for visual clarity
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }

            match step_result {
                StepResult::RepoPath(path) => {
                    context.current_repo_path = Some(path);
                }
                StepResult::Challenges(_step_challenges) => {
                    // Challenges are handled by GameData, ignore here
                }
                StepResult::ScannedFiles(files) => {
                    context.scanned_files = Some(files);
                }
                StepResult::Chunks(chunks) => {
                    context.chunks = Some(chunks);
                }
                StepResult::Skipped => {
                    // Continue to next step
                }
            }
        }

        Ok(())
    }
}
