use std::collections::HashMap;
use serde::Deserialize;

pub const ASSET_BASE: &str = "https://images-static.iracing.com";

pub type CarAssetsMap = HashMap<String, CarAssets>;

#[derive(Debug, Clone, Deserialize)]
pub struct CarAssets {
    pub car_id: i32,
    folder: String,
    logo: Option<String>,
    small_image: String,
}

pub type TrackAssetsMap = HashMap<String, TrackAssets>;

#[derive(Debug, Clone, Deserialize)]
pub struct TrackAssets {
    pub track_id: i32,
    folder: String,
    logo: Option<String>,
    small_image: String,
}

impl CarAssets {
    pub fn logo_url(&self) -> Option<String> {
        Some(format!("{}{}", ASSET_BASE, self.logo.as_ref()?))
    }

    pub fn small_image_url(&self) -> String {
        format!("{}{}/{}", ASSET_BASE, self.folder, self.small_image)
    }
}

impl TrackAssets {
    pub fn logo_url(&self) -> Option<String> {
        Some(format!("{}{}", ASSET_BASE, self.logo.as_ref()?))
    }

    pub fn small_image_url(&self) -> String {
        format!("{}{}/{}", ASSET_BASE, self.folder, self.small_image)
    }
}
