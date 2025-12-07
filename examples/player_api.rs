use leetify::{Client, Id};

#[tokio::main]
async fn main() -> Result<(), leetify::Error> {
    // Create a client
    let client = Client::new();

    // Create a Player instance using the extended API
    // This stores the id so you don't need to pass it to each method
    let player = client.player(Id::Steam64("76561198283431555".into()));

    // Now we can call methods without passing the id each time
    println!("Fetching player profile...");
    let profile = player.profile().await?;
    println!("Player: {}", profile.name);
    println!("Winrate: {:.2}%", profile.winrate * 100.0);
    println!("Total matches: {}", profile.total_matches);

    println!("\nFetching match history...");
    let matches = player.matches().await?;
    println!("Found {} matches", matches.len());

    if let Some(first_match) = matches.first() {
        println!("Most recent match:");
        println!("  ID: {}", first_match.id);
        println!("  Map: {}", first_match.map_name);
        println!("  Finished: {}", first_match.finished_at);
    }

    // You can also create a Player directly
    use leetify::Player;
    let player2 = Player::new(
        Id::Leetify("5ea07280-2399-4c7e-88ab-f2f7db0c449f".into()),
        &client,
    );

    println!("\nFetching profile for Leetify ID...");
    let profile2 = player2.profile().await?;
    println!("Player: {}", profile2.name);

    Ok(())
}
