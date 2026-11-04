use sigmaos_apps::{AppsComponent, Config, DefaultAppProfile, Component};

#[test]
fn test_default_profile_count_aaa() {
    let profile = DefaultAppProfile::sigma_default();
    assert_eq!(profile.count(), 8);
}

#[test]
fn test_default_profile_find_browser_aaa() {
    let profile = DefaultAppProfile::sigma_default();
    let browser = profile.find("web-browser");
    assert!(browser.is_some());
    assert_eq!(browser.unwrap().id, "browser");
    assert_eq!(browser.unwrap().package, "firefox");
}

#[test]
fn test_default_profile_all_have_packages_aaa() {
    let profile = DefaultAppProfile::sigma_default();
    for app in profile.all() {
        assert!(!app.package.is_empty());
        assert!(!app.name.is_empty());
    }
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = AppsComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = AppsComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-apps");
}
