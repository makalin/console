use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Dashboard {
    #[serde(rename = "section")]
    pub sections: Vec<Section>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Section {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "$value")]
    pub content: Vec<SectionContent>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SectionContent {
    #[serde(rename = "message")]
    Message { #[serde(rename = "$text")] text: String },
    #[serde(rename = "warning")]
    Warning { #[serde(rename = "$text")] text: String },
    #[serde(rename = "tire")]
    Tire { #[serde(rename = "@pressure")] pressure: u32, #[serde(rename = "@location")] location: String },
    #[serde(rename = "arrival")]
    Arrival { #[serde(rename = "$text")] text: String },
    #[serde(rename = "map")]
    Map { #[serde(rename = "$value")] content: Vec<MapContent> },
    #[serde(rename = "speed")]
    Speed { #[serde(rename = "@value")] value: u32, #[serde(rename = "@unit")] unit: String },
    #[serde(rename = "rpm")]
    Rpm { #[serde(rename = "@value")] value: u32 },
    #[serde(rename = "player")]
    Player { #[serde(rename = "$value")] content: Vec<PlayerContent> },
    #[serde(rename = "time")]
    Time { #[serde(rename = "$text")] text: String },
    #[serde(rename = "totalDistance")]
    TotalDistance { #[serde(rename = "@value")] value: f32, #[serde(rename = "@unit")] unit: String },
    #[serde(rename = "lap")]
    Lap { #[serde(rename = "@distance")] distance: f32, #[serde(rename = "@unit")] unit: String, #[serde(rename = "@number")] number: u32 },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MapContent {
    #[serde(rename = "route")]
    pub route: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PlayerContent {
    #[serde(rename = "status")]
    Status { #[serde(rename = "$text")] text: String },
    #[serde(rename = "track")]
    Track { #[serde(rename = "$text")] text: String },
    #[serde(rename = "volume")]
    Volume { #[serde(rename = "@level")] level: String },
}

impl Dashboard {
    pub fn from_xml(xml: &str) -> Result<Self, quick_xml::de::DeError> {
        quick_xml::de::from_str(xml)
    }
}

pub mod widgets; 