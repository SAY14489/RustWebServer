use web_server::{Handler, Server, StaticFile};
mod tests;

#[tokio::main]
async fn main() {
    println!("Choose a test to run:");
    println!("1. Basic server test");
    println!("2. Dynamic Handler test");
    println!("3. Duplicate routes test");
    println!("4. Nonexistent file test");
    println!("5. Multiple routes test");
    println!("6. Overhead Measurement Test");
    println!("\nEnter number (1-6):");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => tests::test_basic_server().await,
        "2" => tests::test_dynamic_handler().await,
        "3" => tests::test_duplicate_routes().await,
        "4" => tests::test_nonexistent_file().await,
        "5" => tests::test_multiple_routes().await,
        "6" => tests::test_overhead_measurement().await,
        _ => println!("Invalid choice")
    }
}