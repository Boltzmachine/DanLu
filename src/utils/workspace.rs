use std::fs::create_dir;

pub struct Workspace {
    pub name: String,
}

impl Workspace {
    pub fn new(name: String) -> Result<Self> {
        create_dir(name)?;
        Ok(Self { name })
    }
}
