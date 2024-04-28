use serde::Deserialize;

pub trait TableView<const N: usize> {
    fn columns(&self) -> [&'static str; N];
    fn to_row(self) -> Vec<String>;
}

pub const NUM_COLUMNS: usize = 9;
const COLUMNS: [&str; NUM_COLUMNS] = [
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
    fn columns(&self) -> [&'static str; NUM_COLUMNS] {
        COLUMNS
    }

    fn to_row(self) -> Vec<String> {
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
        vec![
            site, ip, device, resolution, browser, time, country, state, city,
        ]
    }
}

impl TableView<NUM_COLUMNS> for Users {
    fn columns(&self) -> [&'static str; NUM_COLUMNS] {
        COLUMNS
    }

    fn to_row(self) -> Vec<String> {
        unimplemented!()
    }
}
