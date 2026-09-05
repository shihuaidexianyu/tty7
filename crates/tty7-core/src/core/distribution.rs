//! Distribution identity shared by the GUI updater and remote-server installer.
//! Change workspace.package.repository in Cargo.toml when moving the release
//! source; dependency pins and upstream attribution are deliberately separate.

pub const REPOSITORY_URL: &str = env!("CARGO_PKG_REPOSITORY");
pub const STABLE_RELEASE_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/releases/latest");
pub const NIGHTLY_RELEASE_URL: &str =
    concat!(env!("CARGO_PKG_REPOSITORY"), "/releases/tag/nightly");
pub const RELEASE_DOWNLOAD_BASE: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/releases/download");
pub const DOCS_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "#readme");
pub const ISSUES_URL: &str = concat!(env!("CARGO_PKG_REPOSITORY"), "/issues/new");

pub fn release_api_url(suffix: &str) -> String {
    let repository = REPOSITORY_URL
        .strip_prefix("https://github.com/")
        .expect("workspace.package.repository must name the GitHub release repository");
    format!("https://api.github.com/repos/{repository}/releases/{suffix}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_configs_follow_nightly_but_explicit_channel_choices_survive() {
        use crate::core::config::{Config, UpdateChannel};
        assert_eq!(Config::default().update_channel, UpdateChannel::Nightly);
        let stable: Config = serde_json::from_str(r#"{"update_channel":"stable"}"#).unwrap();
        assert_eq!(stable.update_channel, UpdateChannel::Stable);
        let nightly: Config = serde_json::from_str(r#"{"update_channel":"nightly"}"#).unwrap();
        assert_eq!(nightly.update_channel, UpdateChannel::Nightly);
    }

    #[test]
    fn every_update_surface_uses_the_fork() {
        let repository = "https://github.com/shihuaidexianyu/tty7";
        assert_eq!(REPOSITORY_URL, repository);
        assert_eq!(STABLE_RELEASE_URL, format!("{repository}/releases/latest"));
        assert_eq!(
            NIGHTLY_RELEASE_URL,
            format!("{repository}/releases/tag/nightly")
        );
        assert_eq!(
            RELEASE_DOWNLOAD_BASE,
            format!("{repository}/releases/download")
        );
        assert_eq!(DOCS_URL, format!("{repository}#readme"));
        assert_eq!(ISSUES_URL, format!("{repository}/issues/new"));
        assert_eq!(
            release_api_url("latest"),
            "https://api.github.com/repos/shihuaidexianyu/tty7/releases/latest"
        );
        assert_eq!(
            release_api_url("tags/nightly"),
            "https://api.github.com/repos/shihuaidexianyu/tty7/releases/tags/nightly"
        );
    }
}
