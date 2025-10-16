use gittype::domain::models::GitRepository;
use gittype::presentation::tui::screens::title_screen::TitleScreenData;
use gittype::presentation::game::ScreenDataProvider;
use gittype::Result;

pub struct MockTitleScreenDataProvider;

impl ScreenDataProvider for MockTitleScreenDataProvider {
    fn provide(&self) -> Result<Box<dyn std::any::Any>> {
        let challenge_counts = [10, 25, 40, 30, 15]; // Easy, Normal, Hard, Wild, Zen

        let git_repository = Some(GitRepository {
            user_name: "unhappychoice".to_string(),
            repository_name: "gittype".to_string(),
            remote_url: "https://github.com/unhappychoice/gittype.git".to_string(),
            branch: Some("main".to_string()),
            commit_hash: Some("abc1234567890def".to_string()),
            is_dirty: false,
            root_path: None,
        });

        let data = TitleScreenData {
            challenge_counts,
            git_repository,
        };
        Ok(Box::new(data))
    }
}
