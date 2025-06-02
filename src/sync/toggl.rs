pub fn sync() -> anyhow::Result<String> {
    Ok(reqwest::blocking::get("https://google.com")?.json()?)
}
