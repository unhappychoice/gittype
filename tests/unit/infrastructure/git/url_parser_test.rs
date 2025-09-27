#[cfg(test)]
mod tests {
    use gittype::infrastructure::git::UrlParser;

    #[test]
    fn test_parse_short_format() {
        let repo_info = UrlParser::parse_repo_url("owner/repo").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "owner");
        assert_eq!(repo_info.name, "repo");
    }

    #[test]
    fn test_parse_https_format() {
        let repo_info = UrlParser::parse_repo_url("https://github.com/rust-lang/rust").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "rust-lang");
        assert_eq!(repo_info.name, "rust");
    }

    #[test]
    fn test_parse_https_format_with_git_suffix() {
        let repo_info = UrlParser::parse_repo_url("https://github.com/microsoft/vscode.git").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "microsoft");
        assert_eq!(repo_info.name, "vscode");
    }

    #[test]
    fn test_parse_ssh_format() {
        let repo_info = UrlParser::parse_repo_url("git@github.com:unhappychoice/gittype.git").unwrap();
        assert_eq!(repo_info.origin, "github.com");
        assert_eq!(repo_info.owner, "unhappychoice");
        assert_eq!(repo_info.name, "gittype");
    }

    #[test]
    fn test_parse_invalid_format() {
        assert!(UrlParser::parse_repo_url("not a repo").is_err());
        assert!(UrlParser::parse_repo_url("owner").is_err());
    }
}