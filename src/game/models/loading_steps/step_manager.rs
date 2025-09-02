use super::{
    CloningStep, ExecutionContext, ExtractingStep, FinalizingStep, GeneratingStep, ScanningStep,
    Step, StepResult,
};
use crate::game::screens::loading_screen::ProgressReporter;
use crate::game::Challenge;
use crate::Result;

pub struct StepManager {
    steps: Vec<Box<dyn Step>>,
}

impl StepManager {
    pub fn new() -> Self {
        Self {
            steps: vec![
                Box::new(CloningStep),
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

    pub fn execute_pipeline(&self, context: &mut ExecutionContext) -> Result<Vec<Challenge>> {
        let mut challenges: Option<Vec<Challenge>> = None;

        for step in &self.steps {
            // Skip step if it can be skipped
            if step.can_skip(context) {
                continue;
            }

            // Set current step for progress reporting
            if let Some(screen) = context.loading_screen {
                screen.set_step(step.step_type());
            }

            // Execute step
            match step.execute(context)? {
                StepResult::RepoPath(path) => {
                    context.current_repo_path = Some(path);
                }
                StepResult::Challenges(step_challenges) => {
                    challenges = Some(step_challenges);
                }
                StepResult::Skipped => {
                    // Continue to next step
                }
            }
        }

        challenges.ok_or_else(|| {
            crate::GitTypeError::ExtractionFailed("No challenges generated".to_string())
        })
    }
}
