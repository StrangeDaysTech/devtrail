use anyhow::Result;

use crate::self_update;

pub fn run(method: &str) -> Result<()> {
    let method = self_update::parse_method(method);
    self_update::perform_update(method)
}
