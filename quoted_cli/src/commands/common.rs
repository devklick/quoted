pub trait RunCommand {
    async fn run(self) -> Result<(), String>;
}
