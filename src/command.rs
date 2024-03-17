pub trait Command {
    fn execute(&self, args: &[String]) -> anyhow::Result<()>;
}