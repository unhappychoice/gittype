#[cfg(test)]
mod tests {
    use gittype::infrastructure::git::git_repository_ref_parser::GitRepositoryRefParser;

    #[test]
    fn test_parse_short_format() {
        let repo_ref = GitRepositoryRefParser::parse("owner/repo").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "owner");
        assert_eq!(repo_ref.name, "repo");
    }

    #[test]
    fn test_parse_short_format_with_underscore() {
        let repo_ref = GitRepositoryRefParser::parse("unhappy_choice/git_type").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "unhappy_choice");
        assert_eq!(repo_ref.name, "git_type");
    }

    #[test]
    fn test_parse_https_format() {
        let repo_ref = GitRepositoryRefParser::parse("https://github.com/rust-lang/rust").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "rust-lang");
        assert_eq!(repo_ref.name, "rust");
    }

    #[test]
    fn test_parse_https_format_with_git_suffix() {
        let repo_ref = GitRepositoryRefParser::parse("https://github.com/microsoft/vscode.git").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "microsoft");
        assert_eq!(repo_ref.name, "vscode");
    }

    #[test]
    fn test_parse_https_format_with_extra_path() {
        let repo_ref = GitRepositoryRefParser::parse("https://github.com/torvalds/linux/tree/master").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "torvalds");
        assert_eq!(repo_ref.name, "linux");
    }

    #[test]
    fn test_parse_ssh_format() {
        let repo_ref = GitRepositoryRefParser::parse("git@github.com:unhappychoice/gittype.git").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "unhappychoice");
        assert_eq!(repo_ref.name, "gittype");
    }

    #[test]
    fn test_parse_ssh_format_without_git_suffix() {
        let repo_ref = GitRepositoryRefParser::parse("git@github.com:user/repository").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "user");
        assert_eq!(repo_ref.name, "repository");
    }

    #[test]
    fn test_parse_ssh_format_custom_host() {
        let repo_ref = GitRepositoryRefParser::parse("git@gitlab.com:user/project.git").unwrap();
        assert_eq!(repo_ref.origin, "gitlab.com");
        assert_eq!(repo_ref.owner, "user");
        assert_eq!(repo_ref.name, "project");
    }

    #[test]
    fn test_parse_https_format_custom_host() {
        let repo_ref = GitRepositoryRefParser::parse("https://gitlab.com/user/project").unwrap();
        assert_eq!(repo_ref.origin, "gitlab.com");
        assert_eq!(repo_ref.owner, "user");
        assert_eq!(repo_ref.name, "project");
    }

    #[test]
    fn test_parse_invalid_format_no_slash() {
        let result = GitRepositoryRefParser::parse("just-a-name");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_format_empty_string() {
        let result = GitRepositoryRefParser::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_format_with_spaces() {
        let result = GitRepositoryRefParser::parse("not a repo");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_format_single_word() {
        let result = GitRepositoryRefParser::parse("owner");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_ssh_format_no_colon() {
        let result = GitRepositoryRefParser::parse("git@github.com/user/repo");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_domain_slash_path_as_short_format() {
        // "github.com/user" は short format として解釈される（owner/name パターン）
        let repo_ref = GitRepositoryRefParser::parse("github.com/user").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "github.com");
        assert_eq!(repo_ref.name, "user");
    }

    #[test]
    fn test_parse_invalid_https_format_malformed_url() {
        let result = GitRepositoryRefParser::parse("https://");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_ssh_format_malformed() {
        let result = GitRepositoryRefParser::parse("git@github.com:");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_https_format_with_port() {
        let repo_ref = GitRepositoryRefParser::parse("https://github.com:443/user/repo").unwrap();
        assert_eq!(repo_ref.origin, "github.com:443");
        assert_eq!(repo_ref.owner, "user");
        assert_eq!(repo_ref.name, "repo");
    }

    #[test]
    fn test_parse_edge_case_numeric_names() {
        let repo_ref = GitRepositoryRefParser::parse("123/456").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "123");
        assert_eq!(repo_ref.name, "456");
    }

    #[test]
    fn test_parse_edge_case_hyphenated_names() {
        let repo_ref = GitRepositoryRefParser::parse("my-org/my-project").unwrap();
        assert_eq!(repo_ref.origin, "github.com");
        assert_eq!(repo_ref.owner, "my-org");
        assert_eq!(repo_ref.name, "my-project");
    }
}