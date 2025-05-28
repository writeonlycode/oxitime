pub fn post() -> anyhow::Result<String> {
    Ok(reqwest::blocking::get("https://google.com")?.json()?)
}
