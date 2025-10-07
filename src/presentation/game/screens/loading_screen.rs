use crate::domain::events::EventBus;
use crate::domain::models::ExtractionOptions;
use crate::domain::models::{Challenge, GitRepository};
use crate::presentation::game::events::ExitRequested;
use crate::presentation::game::models::{ExecutionContext, StepManager, StepType};
use crate::presentation::game::views::LoadingMainView;
use crate::presentation::game::{
    GameData, RenderBackend, Screen, ScreenDataProvider, ScreenType, UpdateStrategy,
};
use crate::{GitTypeError, Result};
use ratatui::Frame;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex, RwLock,
};
use std::thread;
use std::time::Duration;

pub trait ProgressReporter: Sync {
    fn set_step(&self, step_type: StepType);
    fn set_current_file(&self, file: Option<String>);
    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        current_file: Option<String>,
    );
    fn finish(&self) -> Result<()> {
        Ok(())
    }
}

pub struct NoOpProgressReporter;

impl ProgressReporter for NoOpProgressReporter {
    fn set_step(&self, _step_type: StepType) {}
    fn set_current_file(&self, _file: Option<String>) {}
    fn set_file_counts(
        &self,
        _step_type: StepType,
        _processed: usize,
        _total: usize,
        _current_file: Option<String>,
    ) {
    }
}

#[derive(Clone)]
pub struct LoadingScreenState {
    pub current_step: Arc<RwLock<StepType>>,
    pub step_progress: Arc<RwLock<std::collections::HashMap<StepType, StepProgress>>>,
    pub spinner_index: Arc<AtomicUsize>,
    pub should_stop: Arc<AtomicBool>,
    pub repo_info: Arc<RwLock<Option<String>>>,
    pub all_steps: Arc<RwLock<Vec<StepInfo>>>,
}

#[derive(Clone, Debug)]
pub struct StepProgress {
    pub processed: usize,
    pub total: usize,
    pub progress: f64,
}

#[derive(Clone, Debug)]
pub struct StepInfo {
    pub step_type: StepType,
    pub step_number: usize,
    pub step_name: String,
    pub description: String,
}

pub struct LoadingScreen {
    state: LoadingScreenState,
    render_handle: Option<thread::JoinHandle<Result<()>>>,
    event_bus: EventBus,
    game_data: Arc<Mutex<GameData>>,
}

#[derive(Clone)]
pub struct ProcessingResult {
    pub challenges: Vec<Challenge>,
    pub git_repository: Option<GitRepository>,
}

pub struct ProcessingParams {
    pub repo_spec: Option<String>,
    pub repo_path: Option<PathBuf>,
    pub extraction_options: ExtractionOptions,
}

pub struct LoadingScreenData {
    pub processing_params: Option<ProcessingParams>,
}

pub struct LoadingScreenDataProvider {
    game_data: Arc<Mutex<GameData>>,
}

impl ScreenDataProvider for LoadingScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let processing_params = self.game_data.lock()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to lock game data: {}", e)))?
            .processing_parameters()
            .map(|(repo_spec, repo_path, extraction_options)| {
                ProcessingParams {
                    repo_spec,
                    repo_path,
                    extraction_options,
                }
            });

        Ok(Box::new(LoadingScreenData { processing_params }))
    }
}

impl LoadingScreen {
    pub fn new(event_bus: EventBus) -> Self {
        let step_manager = Arc::new(StepManager::new());

        let steps_info: Vec<StepInfo> = step_manager
            .get_all_steps()
            .iter()
            .map(|step| StepInfo {
                step_type: step.step_type(),
                step_number: step.step_number(),
                step_name: step.step_name().to_string(),
                description: step.description().to_string(),
            })
            .collect();

        let state = LoadingScreenState {
            current_step: Arc::new(RwLock::new(StepType::Cloning)),
            step_progress: Arc::new(RwLock::new(std::collections::HashMap::new())),
            spinner_index: Arc::new(AtomicUsize::new(0)),
            should_stop: Arc::new(AtomicBool::new(false)),
            repo_info: Arc::new(RwLock::new(None)),
            all_steps: Arc::new(RwLock::new(steps_info)),
        };

        Self {
            state,
            render_handle: None,
            event_bus,
            game_data: GameData::instance(),
        }
    }

    pub fn show_initial(&mut self) -> Result<()> {
        self.start_rendering()?;
        Ok(())
    }

    fn start_rendering(&mut self) -> Result<()> {
        // NOTE: In ScreenManager mode, don't create separate terminal
        // Use the existing ratatui rendering through ScreenManager
        Ok(())
    }

    fn start_background_processing(
        &mut self,
        repo_spec: Option<&str>,
        repo_path: Option<&PathBuf>,
        extraction_options: ExtractionOptions,
    ) -> Result<()> {
        let state = self.state.clone();
        let repo_spec_owned = repo_spec.map(|s| s.to_string());
        let repo_path_owned = repo_path.cloned();
        let event_bus = self.event_bus.clone();

        thread::spawn(move || {
            let mut loading_screen = LoadingScreen::new(event_bus);

            loading_screen.state = state;

            match loading_screen.process_repository(
                repo_spec_owned.as_deref(),
                repo_path_owned.as_ref(),
                &extraction_options,
            ) {
                Ok(ProcessingResult {
                    challenges: _,
                    git_repository: _,
                }) => {
                    // Challenges and git_repository are already stored in GameData
                    // by GeneratingStep and FinalizingStep respectively
                    log::info!("Repository processing completed successfully");
                }
                Err(e) => {
                    log::error!("Repository processing failed: {}", e);
                    if let Ok(mut data) = loading_screen.game_data.lock() {
                        data.mark_failed(format!("Repository processing failed: {}", e));
                    }
                }
            }

            let _ = loading_screen.cleanup();
        });

        Ok(())
    }

    pub fn set_repo_info(&self, repo_info: String) -> Result<()> {
        if let Ok(mut info) = self.state.repo_info.write() {
            *info = Some(repo_info);
        }
        Ok(())
    }

    pub fn set_git_repository(
        &self,
        git_repository: &GitRepository,
    ) -> Result<()> {
        let mut parts = vec![format!(
            "📁 {}/{}",
            git_repository.user_name, git_repository.repository_name
        )];

        if let Some(ref branch) = git_repository.branch {
            parts.push(format!("🌿 {}", branch));
        }

        if let Some(ref commit) = git_repository.commit_hash {
            parts.push(format!("📝 {}", &commit[..8]));
        }

        let status_symbol = if git_repository.is_dirty {
            "⚠️"
        } else {
            "✓"
        };
        parts.push(status_symbol.to_string());

        let git_text = parts.join(" • ");

        if let Ok(mut info) = self.state.repo_info.write() {
            *info = Some(git_text);
        }
        Ok(())
    }

    pub fn show_completion(&mut self) -> Result<()> {
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = StepType::Completed;
        }

        thread::sleep(Duration::from_millis(800));

        self.cleanup()?;

        Ok(())
    }

    pub fn show_completion_without_cleanup(&self) -> Result<()> {
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = StepType::Completed;
        }

        thread::sleep(Duration::from_millis(500));

        Ok(())
    }

    pub fn process_repository(
        &mut self,
        repo_spec: Option<&str>,
        repo_path: Option<&PathBuf>,
        options: &ExtractionOptions,
    ) -> Result<ProcessingResult> {
        self.show_initial()?;

        let step_manager = StepManager::new();

        let mut context = ExecutionContext {
            repo_spec,
            repo_path,
            extraction_options: Some(options),
            loading_screen: Some(self),
            current_repo_path: None,
            git_repository: None,
            scanned_files: None,
            chunks: None,
            cache_used: false,
        };

        match step_manager.execute_pipeline(&mut context) {
            Ok(()) => {
                // Show completion
                let _ = self.show_completion_without_cleanup();

                // Git repository is now stored in GameData, so just return empty result
                Ok(ProcessingResult {
                    challenges: Vec::new(), // Challenges are stored in GameData
                    git_repository: None,   // Git repository is stored in GameData
                })
            }
            Err(e) => {
                let _ = self.cleanup();
                Err(e)
            }
        }
    }

    fn draw_ui_static(frame: &mut Frame, state: &LoadingScreenState) {
        LoadingMainView::render(frame, state);
    }

    pub fn cleanup(&mut self) -> Result<()> {
        self.state.should_stop.store(true, Ordering::Relaxed);

        if let Some(handle) = self.render_handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }
}

impl Drop for LoadingScreen {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

impl ProgressReporter for LoadingScreen {
    fn set_step(&self, step_type: StepType) {
        if let Ok(mut current_step) = self.state.current_step.write() {
            *current_step = step_type;
        }
    }

    fn set_current_file(&self, _file: Option<String>) {
        // LoadingScreen doesn't display individual files
    }

    fn set_file_counts(
        &self,
        step_type: StepType,
        processed: usize,
        total: usize,
        _current_file: Option<String>,
    ) {
        let new_progress = if total > 0 {
            processed as f64 / total as f64
        } else {
            0.0
        };

        if let Ok(mut step_progress) = self.state.step_progress.write() {
            let should_update = if let Some(existing) = step_progress.get(&step_type) {
                new_progress > existing.progress
            } else {
                true
            };

            if should_update {
                step_progress.insert(
                    step_type,
                    StepProgress {
                        processed,
                        total,
                        progress: new_progress,
                    },
                );
            }
        }
    }
}

impl Screen for LoadingScreen {
    fn get_type(&self) -> ScreenType {
        ScreenType::Loading
    }

    fn default_provider() -> Box<dyn ScreenDataProvider>
    where
        Self: Sized,
    {
        Box::new(LoadingScreenDataProvider {
            game_data: GameData::instance(),
        })
    }

    fn get_render_backend(&self) -> RenderBackend {
        RenderBackend::Ratatui
    }

    fn init_with_data(&mut self, data: Box<dyn std::any::Any>) -> Result<()> {
        let loading_data = data.downcast::<LoadingScreenData>()?;

        let params = loading_data.processing_params
            .ok_or_else(|| GitTypeError::ScreenInitializationError("No processing parameters found in LoadingScreenData".to_string()))?;

        self.start_background_processing(
            params.repo_spec.as_deref(),
            params.repo_path.as_ref(),
            params.extraction_options,
        )?;

        self.show_initial()
    }


    fn handle_key_event(
        &mut self,
        key_event: crossterm::event::KeyEvent,
    ) -> Result<()> {
        use crossterm::event::{KeyCode, KeyModifiers};

        if key_event.code == KeyCode::Char('c')
            && key_event.modifiers.contains(KeyModifiers::CONTROL)
        {
            self.event_bus.publish(ExitRequested);
        }

        Ok(())
    }

    fn render_crossterm_with_data(
        &mut self,
        _stdout: &mut std::io::Stdout,
    ) -> Result<()> {
        Ok(())
    }

    fn render_ratatui(&mut self, frame: &mut ratatui::Frame) -> Result<()> {
        Self::draw_ui_static(frame, &self.state);
        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        self.state.should_stop.store(true, Ordering::Relaxed);

        if let Some(handle) = self.render_handle.take() {
            let _ = handle.join();
        }

        Ok(())
    }

    fn get_update_strategy(&self) -> UpdateStrategy {
        use std::time::Duration;
        UpdateStrategy::TimeBased(Duration::from_millis(16))
    }

    fn update(&mut self) -> Result<bool> {
        if self.game_data.lock()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to lock game data: {}", e)))?
            .completed() {
            if let Ok(mut current_step) = self.state.current_step.write() {
                *current_step = StepType::Completed;
            }

            return Ok(false);
        }

        if self.game_data.lock()
            .map_err(|e| GitTypeError::TerminalError(format!("Failed to lock game data: {}", e)))?
            .failed() {
            return Ok(false);
        }

        let current_index = self.state.spinner_index.load(Ordering::Relaxed);

        self.state
            .spinner_index
            .store((current_index + 1) % 10, Ordering::Relaxed);
        Ok(true)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
