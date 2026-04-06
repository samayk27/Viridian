use dirs;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    Ollama,
    Anthropic,
    OpenAI,
}

#[derive(Deserialize)]
pub struct Llm {
    provider: LlmProvider,
    model: String,
    base_url: String,
    max_tokens: u32,
}

#[derive(Deserialize)]
pub struct Permissions {
    auto_approve_all: bool,
    auto_approve: Vec<String>,
    always_deny: Vec<String>,
}

#[derive(Deserialize)]
pub struct Shell {
    timeout_seconds: u32,
    shell_binary: String,
}

#[derive(Deserialize)]
pub struct Memory {
    db_path: String,
    context_window_chars: u32,
    auto_compact: bool,
}

#[derive(Deserialize)]
pub struct Ui {
    stream_output: bool,
    syntax_highlight: bool,
}

#[derive(Deserialize)]
pub struct McpServer {
    name: String,
    transport: String,
    command: String,
    args: Vec<String>,
    enabled: bool,
}

#[derive(Deserialize)]
pub struct McpConfig {
    pub servers: Vec<McpServer>,
}

#[derive(Deserialize)]
pub struct Config {
    pub llm: Llm,
    pub permissions: Permissions,
    pub shell: Shell,
    pub memory: Memory,
    pub ui: Ui,
    pub mcp: McpConfig,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Config directory not found"))?
            .join("viridian")
            .join("config.toml");
        let config_content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}
