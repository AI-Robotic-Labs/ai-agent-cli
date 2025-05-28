use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub skills: Vec<String>,
    pub config: HashMap<String, String>,
    pub state: AgentState,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AgentState {
    Created,
    Configured,
    Running,
    Suspended,
    Shutdown,
}

pub struct AgentCLI {
    agent: Option<Agent>,
}

impl AgentCLI {
    pub fn new() -> Self {
        AgentCLI { agent: None }
    }

    pub fn create(&mut self, name: &str) -> Result<(), String> {
        if self.agent.is_none() {
            self.agent = Some(Agent {
                id: format!("agent-{}", rand::random::<u64>()),
                name: name.to_string(),
                skills: Vec::new(),
                config: HashMap::new(),
                state: AgentState::Created,
            });
            Ok(())
        } else {
            Err("Agent already exists".to_string())
        }
    }

    pub fn add_skill(&mut self, skill: &str) -> Result<(), String> {
        if let Some(agent) = &mut self.agent {
            if !agent.skills.contains(&skill.to_string()) {
                agent.skills.push(skill.to_string());
                Ok(())
            } else {
                Err(format!("Skill '{}' already exists", skill))
            }
        } else {
            Err("No agent exists".to_string())
        }
    }

    pub fn configure(&mut self, key: &str, value: &str) -> Result<(), String> {
        if let Some(agent) = &mut self.agent {
            agent.config.insert(key.to_string(), value.to_string());
            agent.state = AgentState::Configured;
            Ok(())
        } else {
            Err("No agent exists".to_string())
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        if let Some(agent) = &mut self.agent {
            match agent.state {
                AgentState::Created | AgentState::Configured | AgentState::Suspended => {
                    agent.state = AgentState::Running;
                    Ok(())
                }
                AgentState::Running => Err("Agent is already running".to_string()),
                AgentState::Shutdown => Err("Agent is shutdown".to_string()),
            }
        } else {
            Err("No agent exists".to_string())
        }
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        if let Some(agent) = &mut self.agent {
            agent.state = AgentState::Shutdown;
            let serialized = serde_json::to_string(&agent).map_err(|e| e.to_string())?;
            let mut file = File::create(format!("{}.json", agent.id)).map_err(|e| e.to_string())?;
            file.write_all(serialized.as_bytes()).map_err(|e| e.to_string())?;
            self.agent = None;
            Ok(())
        } else {
            Err("No agent exists".to_string())
        }
    }

    pub fn get_agent(&self) -> Option<&Agent> {
        self.agent.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_agent() {
        let mut cli = AgentCLI::new();
        assert!(cli.create("test-agent").is_ok());
        let agent = cli.get_agent().unwrap();
        assert_eq!(agent.name, "test-agent");
        assert_eq!(agent.state, AgentState::Created);
        assert!(cli.create("another-agent").is_err()); // Should fail: agent exists
    }

    #[test]
    fn test_add_skill() {
        let mut cli = AgentCLI::new();
        cli.create("test-agent").unwrap();
        assert!(cli.add_skill("memory").is_ok());
        assert!(cli.add_skill("memory").is_err()); // Duplicate skill
        let agent = cli.get_agent().unwrap();
        assert_eq!(agent.skills, vec!["memory"]);
    }

    #[test]
    fn test_configure() {
        let mut cli = AgentCLI::new();
        cli.create("test-agent").unwrap();
        assert!(cli.configure("goal", "assist user").is_ok());
        let agent = cli.get_agent().unwrap();
        assert_eq!(agent.config.get("goal"), Some(&"assist user".to_string()));
        assert_eq!(agent.state, AgentState::Configured);
    }

    #[test]
    fn test_run() {
        let mut cli = AgentCLI::new();
        cli.create("test-agent").unwrap();
        assert!(cli.run().is_ok());
        let agent = cli.get_agent().unwrap();
        assert_eq!(agent.state, AgentState::Running);
        assert!(cli.run().is_err()); // Already running
    }

    #[test]
    fn test_shutdown() {
        let mut cli = AgentCLI::new();
        cli.create("test-agent").unwrap();
        assert!(cli.shutdown().is_ok());
        let agent = cli.get_agent();
        assert!(agent.is_none());
    }
}
