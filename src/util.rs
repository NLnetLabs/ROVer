use ureq::Agent;

use crate::constants::APP_VERSION;

pub fn service_base_uri() -> String {
    let host = std::env::var("ROUTINATOR_HOST").expect("Missing environment variable ROUTINATOR_HOST");
    format!("https://{}", host)
}

pub fn http_client() -> Agent {
    ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(&format!("ROVer/{}", APP_VERSION))
        .build()
}