#[allow(clippy::module_inception)]
mod version_service_tests {
    use gittype::domain::services::VersionService;

    #[tokio::test]
    async fn test_version_update_available() {
        let service = VersionService::new().expect("Service creation should succeed with mocks");

        // Test with current version 0.8.0, mock returns 1.0.0
        let (has_update, current_version, latest_version) = service
            .check_with_version("0.8.0")
            .await
            .expect("Version check should succeed with mocks");

        assert_eq!(current_version, "0.8.0");
        assert_eq!(latest_version, "1.0.0"); // Mock returns this version
        assert!(
            has_update,
            "Should have update available when current=0.8.0 and latest=1.0.0"
        );
    }

    #[tokio::test]
    async fn test_version_up_to_date() {
        let service = VersionService::new().expect("Service creation should succeed");

        // Test with current version 1.0.0, mock returns 1.0.0
        let (has_update, current_version, latest_version) = service
            .check_with_version("1.0.0")
            .await
            .expect("Version check should succeed");

        assert_eq!(current_version, "1.0.0");
        assert_eq!(latest_version, "1.0.0");
        assert!(
            !has_update,
            "Should not have update when versions are equal"
        );
    }

    #[tokio::test]
    async fn test_version_newer_than_latest() {
        let service = VersionService::new().expect("Service creation should succeed");

        // Test with current version 2.0.0, mock returns 1.0.0
        let (has_update, current_version, latest_version) = service
            .check_with_version("2.0.0")
            .await
            .expect("Version check should succeed");

        assert_eq!(current_version, "2.0.0");
        assert_eq!(latest_version, "1.0.0");
        assert!(
            !has_update,
            "Should not have update when current is newer than latest"
        );
    }

    #[tokio::test]
    async fn test_patch_version_update() {
        let service = VersionService::new().expect("Service creation should succeed");

        // Test with current version 0.9.9, mock returns 1.0.0
        let (has_update, current_version, latest_version) = service
            .check_with_version("0.9.9")
            .await
            .expect("Version check should succeed");

        assert_eq!(current_version, "0.9.9");
        assert_eq!(latest_version, "1.0.0");
        assert!(
            has_update,
            "Should have update available when 0.9.9 < 1.0.0"
        );
    }

    #[tokio::test]
    async fn test_service_creation() {
        let service = VersionService::new();
        assert!(
            service.is_ok(),
            "Service creation should succeed with mocks enabled"
        );
    }

    #[tokio::test]
    async fn test_multiple_calls_consistency() {
        let service = VersionService::new().expect("Service creation should succeed");

        let result1 = service
            .check_with_version("0.8.0")
            .await
            .expect("First check should succeed");
        let result2 = service
            .check_with_version("0.8.0")
            .await
            .expect("Second check should succeed");

        // Results should be consistent across calls with same input
        assert_eq!(result1.1, result2.1, "Current version should be consistent");
        assert_eq!(result1.2, result2.2, "Latest version should be consistent");
        assert_eq!(
            result1.0, result2.0,
            "Update availability should be consistent"
        );
    }
}
