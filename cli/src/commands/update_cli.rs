use anyhow::Result;

pub fn run() -> Result<()> {
    crate::self_update::perform_update()
}
