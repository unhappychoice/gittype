use std::collections::HashSet;

use gittype::sharing::SharingPlatform;

#[test]
fn sharing_platform_all_lists_every_variant_once() {
    let platforms = SharingPlatform::all();
    let names: HashSet<_> = platforms.iter().map(|p| p.name()).collect();

    assert_eq!(platforms.len(), 4);
    assert_eq!(names.len(), 4);
    assert!(names.contains("X"));
    assert!(names.contains("Reddit"));
    assert!(names.contains("LinkedIn"));
    assert!(names.contains("Facebook"));
}

#[test]
fn sharing_platform_name_matches_variant() {
    assert_eq!(SharingPlatform::X.name(), "X");
    assert_eq!(SharingPlatform::Reddit.name(), "Reddit");
    assert_eq!(SharingPlatform::LinkedIn.name(), "LinkedIn");
    assert_eq!(SharingPlatform::Facebook.name(), "Facebook");
}
