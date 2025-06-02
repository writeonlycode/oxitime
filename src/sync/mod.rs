use anyhow::Result;

mod toggl;

pub fn sync() -> Result<()> {
    toggl::sync()?;
    Ok(())
}
