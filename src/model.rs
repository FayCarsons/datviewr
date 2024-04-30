use serde::Deserialize;

pub trait TableView<const N: usize> {
    fn to_row(self) -> [String; N];
}

pub const NUM_COLUMNS: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Column {
    Site = 0,
    IP = 1,
    Device = 2,
    Resolution = 3,
    Browser = 4,
    Time = 5,
    Country = 6,
    State = 7,
    City = 8,
}

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        use Column::*;
        match value {
            0 => Site,
            1 => IP,
            2 => Device,
            3 => Resolution,
            4 => Browser,
            5 => Time,
            6 => Country,
            7 => State,
            8 => City,
            _ => panic!("INVALID COLUMN ID: {value}"),
        }
    }
}

pub const COLUMNS: [&str; NUM_COLUMNS] = [
    "Site",
    "IP",
    "Device",
    "Resolution",
    "Browser",
    "Time",
    "Country",
    "State",
    "City",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct User {
    site: String,
    ip: String,
    device: String,
    resolution: String,
    browser: String,
    time: String,
    country: String,
    state: String,
    #[serde(default)]
    city: String,
}

pub type Users = Vec<User>;

impl TableView<NUM_COLUMNS> for User {
    fn to_row(self) -> [String; NUM_COLUMNS] {
        let User {
            site,
            ip,
            device,
            resolution,
            browser,
            time,
            country,
            state,
            city,
        } = self;
        [
            site, ip, device, resolution, browser, time, country, state, city,
        ]
    }
}

// DateTime FMT: April 24, 2024 at 4:42:55 PM
