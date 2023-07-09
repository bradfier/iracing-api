use serde::{Deserialize, Serialize};

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
    pub car_id: i64,
    pub car_name: String,
    pub car_name_abbreviated: String,
    pub car_types: Vec<CarType>,
    pub car_weight: i64,
    pub categories: Vec<String>,
    pub created: String,
    pub first_sale: String,
    pub forum_url: Option<String>,
    pub free_with_subscription: bool,
    pub has_headlights: bool,
    pub has_multiple_dry_tire_types: bool,
    pub hp: i64,
    pub is_ps_purchasable: bool,
    pub max_power_adjust_pct: i64,
    pub max_weight_penalty_kg: i64,
    pub min_power_adjust_pct: i64,
    pub package_id: i64,
    pub patterns: i64,
    pub price: f64,
    pub price_display: Option<String>,
    pub retired: bool,
    pub search_filters: String,
    pub sku: i64,
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
    pub cust_id: i64,
    pub email: String,
    pub username: String,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub on_car_name: String,
    pub member_since: String,
    pub last_test_track: i64,
    pub last_test_car: i64,
    pub last_season: i64,
    pub flags: i64,
    pub club_id: i64,
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
    pub other_owned_packages: Vec<i64>,
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
    pub package_id: i64,
    pub content_ids: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Helmet {
    pub pattern: i64,
    pub color1: String,
    pub color2: String,
    pub color3: String,
    pub face_type: i64,
    pub helmet_type: i64,
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
    pub category_id: i64,
    pub category: String,
    pub license_level: i64,
    pub safety_rating: f64,
    pub cpi: f64,
    pub irating: i64,
    pub tt_rating: i64,
    pub mpr_num_races: i64,
    pub color: String,
    pub group_name: String,
    pub group_id: i64,
    pub pro_promotable: bool,
    pub mpr_num_tts: i64,
}

// This likely contains information about bans, chat restrictions etc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restrictions {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suit {
    pattern: i64,
    color1: String,
    color2: String,
    color3: String,
    body_type: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    ai_enabled: bool,
    allow_pitlane_collisions: bool,
    allow_rolling_start: bool,
    allow_standing_start: bool,
    award_exempt: bool,
    category: Category,
    category_id: i64,
    closes: String,
    config_name: Option<String>,
    corners_per_lap: i64,
    created: String,
    first_sale: String,
    free_with_subscription: bool,
    fully_lit: bool,
    grid_stalls: i64,
    has_opt_path: bool,
    has_short_parade_lap: bool,
    has_start_zone: bool,
    has_svg_map: bool,
    is_dirt: bool,
    is_oval: bool,
    is_ps_purchasable: bool,
    lap_scoring: i64,
    latitude: f64,
    location: String,
    longitude: f64,
    max_cars: i64,
    night_lighting: bool,
    nominal_lap_time: f64,
    number_pitstalls: i64,
    opens: String,
    package_id: i64,
    pit_road_speed_limit: Option<i64>,
    price: f64,
    price_display: Option<String>,
    priority: i64,
    purchasable: bool,
    qualify_laps: i64,
    restart_on_left: bool,
    retired: bool,
    search_filters: String,
    site_url: Option<String>,
    sku: i64,
    solo_laps: i64,
    start_on_left: bool,
    supports_grip_compound: bool,
    tech_track: bool,
    time_zone: String,
    track_config_length: f64,
    track_dirpath: String,
    track_id: i64,
    track_name: String,
    track_types: Vec<TrackType>,
    banking: Option<String>,
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
