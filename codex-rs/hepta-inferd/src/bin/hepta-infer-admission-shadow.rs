#[path = "../shadow.rs"]
mod shadow;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    shadow::run_from_env().await
}
