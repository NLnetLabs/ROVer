use serde::Deserialize;

// --- stat.ripe.net/data/as-overview/data.json?resource=ASNNNNN

#[derive(Deserialize, Debug)]
pub struct AsOverviewData {
    pub holder: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AsOverviewResponse {
    pub status: String,
    pub data: AsOverviewData,
}