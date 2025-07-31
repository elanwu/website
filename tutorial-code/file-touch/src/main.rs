use tokio::fs::OpenOptions;
use tokio::io::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Touch a file - create it if it doesn't exist, or update its timestamp if it does
    let filename = "example.txt";
    
    // Using OpenOptions to create the file if it doesn't exist
    // without truncating existing content
    let _file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)
        .await?;
    
    println!("Touched file: {}", filename);
    
    Ok(())
}