// use storage::Storage;
use gui::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // eventually need two way channels for events to go from filesystem to gui
    run().unwrap();
    Ok(())
}
