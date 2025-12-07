use leetify::{Client, DataSource, PlayerId};

#[tokio::main]
async fn main() -> Result<(), leetify::Error> {
    // Create a client without an API key
    let client = Client::new();

    // Example 1: Get player profile by Steam64 ID
    println!("Fetching profile by Steam64 ID...");
    match client
        .get_profile(PlayerId::Steam64("76561198283431555".into()))
        .await
    {
        Ok(profile) => {
            println!("Player: {}", profile.name);
            println!("Winrate: {:.2}%", profile.winrate * 100.0);
            println!("Total matches: {}", profile.total_matches);
            println!("Leetify Rating: {:.2}", profile.rating.aim);
        }
        Err(e) => eprintln!("Error fetching profile: {}", e),
    }

    // Example 2: Get player profile by Leetify ID (UUID format)
    println!("\nFetching profile by Leetify ID...");
    match client
        .get_profile(PlayerId::Leetify(
            "5ea07280-2399-4c7e-88ab-f2f7db0c449f".into(),
        ))
        .await
    {
        Ok(profile) => {
            println!("Player: {}", profile.name);
        }
        Err(e) => eprintln!("Error fetching profile: {}", e),
    }

    // Example 3: Get match history
    println!("\nFetching match history...");
    match client
        .get_profile_matches(PlayerId::Steam64("76561198283431555".into()))
        .await
    {
        Ok(matches) => {
            println!("Found {} matches", matches.len());
            if let Some(first_match) = matches.first() {
                println!("Most recent match: {}", first_match.map_name);
            }
        }
        Err(e) => eprintln!("Error fetching matches: {}", e),
    }

    // Example 4: Get match details by game ID
    println!("\nFetching match details...");
    match client
        .get_match_by_game_id("match-id-here".to_string())
        .await
    {
        Ok(match_details) => {
            println!("Match ID: {}", match_details.id);
            println!("Map: {}", match_details.map_name);
            println!("Players: {}", match_details.stats.len());
        }
        Err(e) => eprintln!("Error fetching match: {}", e),
    }

    // Example 5: Get match by data source
    println!("\nFetching FACEIT match...");
    match client
        .get_match_by_data_source(DataSource::FACEIT, "faceit-match-id")
        .await
    {
        Ok(match_details) => {
            println!("Match ID: {}", match_details.id);
        }
        Err(e) => eprintln!("Error fetching match: {}", e),
    }

    // Example 6: Using the builder pattern
    println!("\nCreating client with builder...");
    let custom_client = Client::builder()
        .api_key("your-api-key-here")
        .timeout(std::time::Duration::from_secs(60))
        .build()?;

    // Example 7: Validate API key
    println!("\nValidating API key...");
    match custom_client.validate_api_key().await {
        Ok(()) => println!("API key is valid!"),
        Err(e) => eprintln!("API key validation failed: {}", e),
    }

    // Example 8: Using PlayerId with automatic conversion
    println!("\nUsing PlayerId with automatic conversion...");
    // String automatically converts to PlayerId:
    // - UUID format (xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx) -> Leetify ID
    // - Numeric strings (15+ digits) -> Steam64 ID
    let id: PlayerId = "76561198283431555".into();
    match client.get_profile(id).await {
        Ok(profile) => println!("Profile fetched: {}", profile.name),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
