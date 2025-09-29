pub mod git_repository_ref_parser;
pub mod local;
pub mod remote;

pub use git_repository_ref_parser::GitRepositoryRefParser;
pub use local::LocalGitRepositoryClient;
pub use remote::RemoteGitRepositoryClient;
