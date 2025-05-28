use agent_cli::{AgentCLI, AgentState};
use std::env;

fn print_usage() {
    println!(
        "Usage:
        agent create <name>          - Create a new agent
        agent skill add <skill>      - Add a skill to the agent
        agent config <key> <value>   - Configure agent settings
        agent run                    - Run the agent
        agent shutdown               - Shutdown and save agent state"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cli = AgentCLI::new();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "create" => {
            if args.len() == 3 {
                match cli.create(&args[2]) {
                    Ok(_) => {
                        let agent = cli.get_agent().unwrap();
                        println!("Created agent: {} (ID: {})", agent.name, agent.id);
                    }
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Usage: agent create <name>");
            }
        }
        "skill" => {
            if args.len() >= 4 && args[2] == "add" {
                match cli.add_skill(&args[3]) {
                    Ok(_) => println!("Added skill '{}' to agent {}", args[3], cli.get_agent().unwrap().name),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Usage: agent skill add <skill>");
            }
        }
        "config" => {
            if args.len() == 4 {
                match cli.configure(&args[2], &args[3]) {
                    Ok(_) => println!("Configured {} = {} for agent {}", args[2], args[3], cli.get_agent().unwrap().name),
                    Err(e) => println!("Error: {}", e),
                }
            } else {
                println!("Usage: agent config <key> <value>");
            }
        }
        "run" => match cli.run() {
            Ok(_) => {
                let agent = cli.get_agent().unwrap();
                println!("Running agent {} (ID: {})", agent.name, agent.id);
                println!("Active skills: {:?}", agent.skills);
                println!("Configuration: {:?}", agent.config);
            }
            Err(e) => println!("Error: {}", e),
        },
        "shutdown" => match cli.shutdown() {
            Ok(_) => println!("Agent shutdown and state saved"),
            Err(e) => println!("Error: {}", e),
        },
        _ => print_usage(),
    }
}