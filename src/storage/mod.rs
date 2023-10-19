pub trait Storage {
    pub fn load(&self) -> Result<()>;
    pub fn get_run(&self) -> Result<Run>;
}