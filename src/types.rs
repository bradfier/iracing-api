use serde::{Deserialize, Serialize};
use std::ops::Deref;
use time::OffsetDateTime;

pub mod assets;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDate(#[serde(with = "time::serde::rfc3339")] OffsetDateTime);

impl Deref for ApiDate {
    type Target = OffsetDateTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
    pub ai_enabled: bool,
    pub allow_number_colors: bool,
    pub allow_number_font: bool,
    pub allow_sponsor1: bool,
    pub allow_sponsor2: bool,
    pub allow_wheel_color: bool,
    pub award_exempt: bool,
    pub car_dirpath: String,
    pub car_id: i32,
    pub car_name: String,
    pub car_name_abbreviated: String,
    pub car_types: Vec<CarType>,
    pub car_weight: i32,
    pub categories: Vec<String>,
    pub created: String,
    pub first_sale: String,
    pub forum_url: Option<String>,
    pub free_with_subscription: bool,
    pub has_headlights: bool,
    pub has_multiple_dry_tire_types: bool,
    pub hp: i32,
    pub is_ps_purchasable: bool,
    pub max_power_adjust_pct: i32,
    pub max_weight_penalty_kg: i32,
    pub min_power_adjust_pct: i32,
    pub package_id: i32,
    pub patterns: i32,
    pub price: f64,
    pub price_display: Option<String>,
    pub retired: bool,
    pub search_filters: String,
    pub sku: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarType {
    pub car_type: String,
}

#[derive(Debug, Clone)]
pub struct Cars {
    pub inner: Vec<Car>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub cust_id: i32,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub on_car_name: String,
    pub member_since: String,
    pub last_test_track: i32,
    pub last_test_car: i32,
    pub last_season: i32,
    pub flags: i32,
    pub club_id: i32,
    pub club_name: String,
    pub connection_type: String,
    pub download_server: String,
    pub last_login: String,
    pub read_comp_rules: String,
    pub account: Account,
    pub helmet: Helmet,
    pub suit: Suit,
    pub licenses: Licenses,
    pub car_packages: Vec<Package>,
    pub track_packages: Vec<Package>,
    pub other_owned_packages: Vec<i32>,
    pub dev: bool,
    pub alpha_tester: bool,
    pub broadcaster: bool,
    pub restrictions: Restrictions,
    pub has_read_comp_rules: bool,
    pub flags_hex: String,
    pub hundred_pct_club: bool,
    pub twenty_pct_discount: bool,
    pub race_official: bool,
    pub ai: bool,
    pub bypass_hosted_password: bool,
    pub read_tc: String,
    pub read_pp: String,
    pub has_read_tc: bool,
    pub has_read_pp: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub ir_dollars: f64,
    pub ir_credits: f64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub package_id: i32,
    pub content_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Helmet {
    pub pattern: i32,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub face_type: i32,
    pub helmet_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Licenses {
    pub oval: License,
    pub road: License,
    pub dirt_oval: License,
    pub dirt_road: License,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub category_id: i32,
    pub category: String,
    pub license_level: i32,
    pub safety_rating: f64,
    pub cpi: f64,
    pub irating: i32,
    pub tt_rating: i32,
    pub mpr_num_races: i32,
    pub color: String,
    pub group_name: String,
    pub group_id: i32,
    pub pro_promotable: bool,
    pub mpr_num_tts: i32,
}

// This likely contains information about bans, chat restrictions etc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restrictions {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suit {
    pattern: i32,
    color1: String,
    color2: String,
    color3: String,
    body_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub ai_enabled: bool,
    pub allow_pitlane_collisions: bool,
    pub allow_rolling_start: bool,
    pub allow_standing_start: bool,
    pub award_exempt: bool,
    pub category: String,
    pub category_id: i32,
    pub closes: String,
    pub config_name: Option<String>,
    pub corners_per_lap: i32,
    pub created: String,
    pub first_sale: String,
    pub free_with_subscription: bool,
    pub fully_lit: bool,
    pub grid_stalls: i32,
    pub has_opt_path: bool,
    pub has_short_parade_lap: bool,
    pub has_start_zone: bool,
    pub has_svg_map: bool,
    pub is_dirt: bool,
    pub is_oval: bool,
    pub is_ps_purchasable: bool,
    pub lap_scoring: i32,
    pub latitude: f64,
    pub location: String,
    pub longitude: f64,
    pub max_cars: i32,
    pub night_lighting: bool,
    pub nominal_lap_time: f64,
    pub number_pitstalls: i32,
    pub opens: String,
    pub package_id: i32,
    pub pit_road_speed_limit: Option<i32>,
    pub price: f64,
    pub price_display: Option<String>,
    pub priority: i32,
    pub purchasable: bool,
    pub qualify_laps: i32,
    pub restart_on_left: bool,
    pub retired: bool,
    pub search_filters: String,
    pub site_url: Option<String>,
    pub sku: i32,
    pub solo_laps: i32,
    pub start_on_left: bool,
    pub supports_grip_compound: bool,
    pub tech_track: bool,
    pub time_zone: String,
    pub track_config_length: f64,
    pub track_dirpath: String,
    pub track_id: i32,
    pub track_name: String,
    pub track_types: Vec<TrackType>,
    pub banking: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackType {
    pub track_type: Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    DirtOval,
    DirtRoad,
    Oval,
    Road,
}

#[derive(Debug, Clone)]
pub struct Tracks {
    pub inner: Vec<Track>,
}

/// Concise series definition as the list of fields is truly vast.
#[derive(Debug, Clone)]
pub struct SeriesList {
    pub inner: Vec<Series>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub active: bool,
    pub driver_changes: bool,
    pub schedules: Vec<Schedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub series_name: String,
    pub season_name: String,
    pub race_lap_limit: Option<u16>,
    pub race_time_limit: Option<u16>,
    pub track: ScheduleTrack,
    pub race_time_descriptors: Vec<SessionGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleTrack {
    pub track_id: i32,
    pub track_name: String,
    pub config_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionGroup {
    pub session_times: Option<Vec<ApiDate>>,
}
