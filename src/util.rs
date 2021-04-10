pub fn service_base_uri() -> String {
    let host = std::env::var("ROUTINATOR_HOST").expect("Missing environment variable ROUTINATOR_HOST");
    format!("https://{}", host)
}
