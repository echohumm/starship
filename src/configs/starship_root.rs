use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn default_profiles() -> IndexMap<String, String> {
    IndexMap::new()
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct StarshipRootConfig {
    #[serde(rename = "$schema")]
    schema: String,
    pub format: String,
    pub success_style: String,
    pub failure_style: String,
    pub root_success_style: String,
    pub root_failure_style: String,
    pub sigint_failure: bool,
    pub right_format: String,
    pub continuation_prompt: String,
    pub scan_timeout: u64,
    pub command_timeout: u64,
    pub add_newline: bool,
    pub follow_symlinks: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palette: Option<String>,
    pub palettes: HashMap<String, Palette>,
    #[serde(rename = "profiles")]
    #[cfg_attr(feature = "config-schema", schemars(default = "default_profiles"))]
    pub user_profiles: IndexMap<String, String>,
    #[serde(skip)]
    pub internal_profiles: IndexMap<String, String>,
}

pub type Palette = HashMap<String, String>;

// List of default prompt order.
pub const PROMPT_ORDER: &[&str] = &[
    "username",
    "hostname",
    "shlvl",
    "directory",
    "vcsh",
    "git_branch",
    "git_commit",
    "git_state",
    "git_metrics",
    "git_status",
    "docker_context",
    "package",
    // ↓ Toolchain version modules ↓
    // (Let's keep these sorted alphabetically)
    "c",
    "cmake",
    "cpp",
    "dotnet",
    "golang",
    "gradle",
    "java",
    "kotlin",
    "maven",
    "nodejs",
    "python",
    "rust",
    "zig",
    // ↑ Toolchain version modules ↑
    "meson",
    "direnv",
    "env_var",
    "custom",
    "sudo",
    "cmd_duration",
    "line_break",
    "jobs",
    "time",
    "status",
    "container",
    "shell",
    "character",
];

// On changes please also update `Default` for the `FullConfig` struct in `mod.rs`
impl Default for StarshipRootConfig {
    fn default() -> Self {
        Self {
            schema: "https://starship.rs/config-schema.json".to_string(),
            format: "$all".to_string(),
            success_style: "green".to_string(),
            failure_style: "red".to_string(),
            root_success_style: "green".to_string(),
            root_failure_style: "red".to_string(),
            sigint_failure: false,
            right_format: String::new(),
            continuation_prompt: "[∙](bright-black) ".to_string(),
            user_profiles: IndexMap::new(),
            internal_profiles: default_profiles(),
            scan_timeout: 30,
            command_timeout: 500,
            add_newline: true,
            follow_symlinks: true,
            palette: None,
            palettes: HashMap::default(),
        }
    }
}
