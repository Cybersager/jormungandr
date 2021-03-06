mod rest;
mod version;

pub use crate::testing::node::configuration::{
    LegacyConfigConverter, LegacyConfigConverterError, LegacyNodeConfigConverter,
};
use crate::testing::{decompress, download_file, CachedReleases, GitHubApi, Release};
pub use jormungandr_lib::interfaces::{
    Explorer, Log, Mempool, NodeConfig, P2p, Policy, Rest, TopicsOfInterest, TrustedPeer,
};
use jortestkit::file;

use assert_fs::fixture::PathChild;
use assert_fs::prelude::*;
use url::Url;

use std::path::PathBuf;

pub use rest::BackwardCompatibleRest;

pub use version::{version_0_8_19, Version};

lazy_static::lazy_static! {
    static ref RELEASES: CachedReleases = {
        let api = GitHubApi::new();
        api.describe_releases().unwrap()
    };
}

pub fn download_last_n_releases(n: u32) -> Vec<Release> {
    RELEASES
        .into_iter()
        .cloned()
        .filter(|x| !x.version_str().starts_with("nightly"))
        .take(n as usize)
        .collect()
}

pub fn get_jormungandr_bin(release: &Release, temp_dir: &impl PathChild) -> PathBuf {
    let asset = RELEASES
        .get_asset_for_current_os_by_version(release.version_str())
        .unwrap()
        .unwrap();

    let url = Url::parse(&asset.download_url()).expect("cannot parse url");
    let file_name = url
        .path_segments()
        .unwrap()
        .last()
        .expect("cannot get last element from path");

    let output = temp_dir.child(&file_name);
    download_file(asset.download_url(), output.path()).expect("cannot download file");
    let release_dir = temp_dir.child(format!("release-{}", release.version()));
    release_dir.create_dir_all().unwrap();
    decompress(output.path(), release_dir.path()).unwrap();
    file::find_file(release_dir.path(), "jormungandr").unwrap()
}
