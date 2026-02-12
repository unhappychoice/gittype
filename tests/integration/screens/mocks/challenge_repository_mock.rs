use gittype::domain::models::{Challenge, GitRepository};
use gittype::domain::repositories::challenge_repository::ChallengeRepositoryInterface;
use gittype::presentation::tui::screens::loading_screen::ProgressReporter;
use gittype::Result;

pub struct MockChallengeRepository;

impl MockChallengeRepository {
    pub fn new() -> Self {
        MockChallengeRepository
    }
}

impl ChallengeRepositoryInterface for MockChallengeRepository {
    fn save_challenges(
        &self,
        _repo: &GitRepository,
        _challenges: &[Challenge],
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<()> {
        Ok(())
    }

    fn load_challenges_with_progress(
        &self,
        _repo: &GitRepository,
        _reporter: Option<&dyn ProgressReporter>,
    ) -> Result<Option<Vec<Challenge>>> {
        Ok(None)
    }

    fn get_cache_stats(&self) -> Result<(usize, u64)> {
        Ok((0, 0))
    }

    fn clear_cache(&self) -> Result<()> {
        Ok(())
    }

    fn invalidate_repository(&self, _repo: &GitRepository) -> Result<bool> {
        Ok(false)
    }

    fn list_cache_keys(&self) -> Result<Vec<String>> {
        Ok(vec![])
    }
}
