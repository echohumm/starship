use indexmap::IndexMap;
use serde::{self, Deserialize, Serialize};

pub mod c;
pub mod cc;
pub mod character;
pub mod cmake;
pub mod cmd_duration;
pub mod container;
pub mod cpp;
pub mod custom;
pub mod directory;
pub mod direnv;
pub mod docker_context;
pub mod dotnet;
pub mod env_var;
pub mod fill;
pub mod git_branch;
pub mod git_commit;
pub mod git_metrics;
pub mod git_state;
pub mod git_status;
pub mod go;
pub mod gradle;
pub mod hostname;
pub mod java;
pub mod jobs;
pub mod kotlin;
pub mod line_break;
pub mod maven;
pub mod meson;
pub mod nix_shell;
pub mod nodejs;
pub mod package;
pub mod python;
pub mod rust;
pub mod shell;
pub mod shlvl;
mod starship_root;
pub mod status;
pub mod sudo;
pub mod time;
pub mod username;
pub mod vcsh;
pub mod zig;

pub use starship_root::*;

#[derive(Serialize, Deserialize, Clone, Default)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FullConfig<'a> {
    // Meta
    #[serde(rename = "$schema")]
    schema: String,
    // Root config
    #[serde(flatten)]
    root: StarshipRootConfig,
    // modules
    #[serde(borrow)]
    c: c::CConfig<'a>,
    #[serde(borrow)]
    character: character::CharacterConfig<'a>,
    #[serde(borrow)]
    cmake: cmake::CMakeConfig<'a>,
    #[serde(borrow)]
    cmd_duration: cmd_duration::CmdDurationConfig<'a>,
    #[serde(borrow)]
    container: container::ContainerConfig<'a>,
    #[serde(borrow)]
    cpp: cpp::CppConfig<'a>,
    #[serde(borrow)]
    directory: directory::DirectoryConfig<'a>,
    #[serde(borrow)]
    direnv: direnv::DirenvConfig<'a>,
    #[serde(borrow)]
    docker_context: docker_context::DockerContextConfig<'a>,
    #[serde(borrow)]
    dotnet: dotnet::DotnetConfig<'a>,
    #[serde(borrow)]
    env_var: IndexMap<String, env_var::EnvVarConfig<'a>>,
    #[serde(borrow)]
    fill: fill::FillConfig<'a>,
    #[serde(borrow)]
    git_branch: git_branch::GitBranchConfig<'a>,
    #[serde(borrow)]
    git_commit: git_commit::GitCommitConfig<'a>,
    #[serde(borrow)]
    git_metrics: git_metrics::GitMetricsConfig<'a>,
    #[serde(borrow)]
    git_state: git_state::GitStateConfig<'a>,
    #[serde(borrow)]
    git_status: git_status::GitStatusConfig<'a>,
    #[serde(borrow)]
    golang: go::GoConfig<'a>,
    #[serde(borrow)]
    gradle: gradle::GradleConfig<'a>,
    #[serde(borrow)]
    hostname: hostname::HostnameConfig<'a>,
    #[serde(borrow)]
    java: java::JavaConfig<'a>,
    #[serde(borrow)]
    jobs: jobs::JobsConfig<'a>,
    #[serde(borrow)]
    kotlin: kotlin::KotlinConfig<'a>,
    line_break: line_break::LineBreakConfig,
    #[serde(borrow)]
    maven: maven::MavenConfig<'a>,
    #[serde(borrow)]
    meson: meson::MesonConfig<'a>,
    #[serde(borrow)]
    nix_shell: nix_shell::NixShellConfig<'a>,
    #[serde(borrow)]
    nodejs: nodejs::NodejsConfig<'a>,
    #[serde(borrow)]
    package: package::PackageConfig<'a>,
    #[serde(borrow)]
    python: python::PythonConfig<'a>,
    #[serde(borrow)]
    rust: rust::RustConfig<'a>,
    #[serde(borrow)]
    shell: shell::ShellConfig<'a>,
    #[serde(borrow)]
    shlvl: shlvl::ShLvlConfig<'a>,
    #[serde(borrow)]
    status: status::StatusConfig<'a>,
    #[serde(borrow)]
    sudo: sudo::SudoConfig<'a>,
    #[serde(borrow)]
    time: time::TimeConfig<'a>,
    #[serde(borrow)]
    username: username::UsernameConfig<'a>,
    #[serde(borrow)]
    vcsh: vcsh::VcshConfig<'a>,
    #[serde(borrow)]
    zig: zig::ZigConfig<'a>,
    #[serde(borrow)]
    custom: IndexMap<String, custom::CustomConfig<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;
    use toml::value::Value;

    #[test]
    fn test_all_modules_in_full_config() {
        let full_cfg = Value::try_from(FullConfig::default()).unwrap();
        let cfg_table = full_cfg.as_table().unwrap();
        for module in ALL_MODULES {
            assert!(cfg_table.contains_key(*module));
        }
    }
}
