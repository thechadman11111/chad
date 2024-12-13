# Ignore all JavaScript files
*.js linguist-ignore

# Ignore a specific directory
/node_modules/* linguist-ignore

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    if let Err(e) = dotenv() {
        eprintln!("Error loading .env file: {}", e);
    }

    let mut runtime = Runtime::new(
        &env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
        &env::var("TWITTER_CONSUMER_KEY").expect("TWITTER_CONSUMER_KEY not set"),
        &env::var("TWITTER_CONSUMER_SECRET").expect("TWITTER_CONSUMER_SECRET not set"),
        &env::var("TWITTER_ACCESS_TOKEN").expect("TWITTER_ACCESS_TOKEN not set"),
        &env::var("TWITTER_ACCESS_TOKEN_SECRET").expect("TWITTER_ACCESS_TOKEN_SECRET not set"),
    );

    let mut instruction_builder = InstructionBuilder::new();
    let character_name = env::var("CHARACTER_NAME")
        .expect("CHARACTER_NAME not set")
        .trim()
        .to_string();

    println!("Running character: {}", character_name);

    if let Err(e) = instruction_builder.build_instructions(&character_name) {
        eprintln!("Error building instructions: {}", e);
        return Err(anyhow::anyhow!("Failed to build instructions"));
    }
    runtime.add_agent(instruction_builder.get_instructions());

    runtime.run_periodically().await?;

    Ok(())
}
