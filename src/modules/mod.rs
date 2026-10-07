// While adding out new module add out module to src/module.rs ALL_MODULES const array also.
mod c;
mod cc;
mod character;
mod cmake;
mod cmd_duration;
mod container;
mod cpp;
pub mod custom;
mod directory;
mod direnv;
mod docker_context;
mod dotnet;
mod env_var;
mod fill;
mod git_branch;
mod git_commit;
mod git_metrics;
mod git_state;
pub mod git_status;
mod golang;
mod gradle;
mod hostname;
mod java;
mod jobs;
mod kotlin;
mod line_break;
mod maven;
mod memory_usage;
mod meson;
mod nix_shell;
mod nodejs;
mod package;
mod python;
mod rust;
mod shell;
mod shlvl;
mod status;
mod sudo;
mod time;
mod username;
mod utils;
mod vcs;
mod vcsh;
mod zig;

mod typst;

use crate::config::ModuleConfig;
use crate::context::{Context, Detected, Shell};
use crate::module::Module;
use std::time::Instant;

pub fn handle<'a>(module: &str, context: &'a Context) -> Option<Module<'a>> {
    let start: Instant = Instant::now();
    let mut m: Option<Module> = {
        match module {
            // Keep these ordered alphabetically.
            // Default ordering is handled in configs/starship_root.rs
            "c" => c::module(context),
            "character" => character::module(context),
            "cmake" => cmake::module(context),
            "cmd_duration" => cmd_duration::module(context),
            "container" => container::module(context),
            "cpp" => cpp::module(context),
            "directory" => directory::module(context),
            "direnv" => direnv::module(context),
            "docker_context" => docker_context::module(context),
            "dotnet" => dotnet::module(context),
            "env_var" => env_var::module(None, context),
            "fill" => fill::module(context),
            "git_branch" => git_branch::module(context),
            "git_commit" => git_commit::module(context),
            "git_metrics" => git_metrics::module(context),
            "git_state" => git_state::module(context),
            "git_status" => git_status::module(context),
            "golang" => golang::module(context),
            "gradle" => gradle::module(context),
            "hostname" => hostname::module(context),
            "java" => java::module(context),
            "jobs" => jobs::module(context),
            "kotlin" => kotlin::module(context),
            "line_break" => line_break::module(context),
            "maven" => maven::module(context),
            "memory_usage" => memory_usage::module(context),
            "meson" => meson::module(context),
            "nix_shell" => nix_shell::module(context),
            "nodejs" => nodejs::module(context),
            "package" => package::module(context),
            "python" => python::module(context),
            "rust" => rust::module(context),
            "shell" => shell::module(context),
            "shlvl" => shlvl::module(context),
            "status" => status::module(context),
            "sudo" => sudo::module(context),
            "time" => time::module(context),
            "typst" => typst::module(context),
            "username" => username::module(context),
            "vcs" => vcs::module(context),
            "vcsh" => vcsh::module(context),
            "zig" => zig::module(context),
            env if env.starts_with("env_var.") => {
                env_var::module(env.strip_prefix("env_var."), context)
            }
            custom if custom.starts_with("custom.") => {
                // SAFETY: We just checked that the module starts with "custom."
                custom::module(custom.strip_prefix("custom.").unwrap(), context)
            }
            _ => {
                eprintln!(
                    "Error: Unknown module {module}. Use starship module --list to list out all supported modules."
                );
                None
            }
        }
    };

    let elapsed = start.elapsed();
    log::trace!("Took {elapsed:?} to compute module {module:?}");
    if elapsed.as_millis() >= 1 {
        // If we take less than 1ms to compute a None, then we will not return a module at all
        // if we have a module: default duration is 0 so no need to change it
        // if we took more than 1ms we want to report that and so--in case we have None currently--
        // need to create an empty module just to hold the duration for that case
        m.get_or_insert_with(|| context.new_module(module)).duration = elapsed;
    }
    m
}

pub fn description(module: &str) -> &'static str {
    match module {
        "aws" => "The current AWS region and profile",
        "azure" => "The current Azure subscription",
        "battery" => "The current charge of the device's battery and its current charging status",
        "buf" => "The currently installed version of the Buf CLI",
        "bun" => "The currently installed version of the Bun",
        "c" => "Your C compiler type",
        "character" => {
            "A character (usually an arrow) beside where the text is entered in your terminal"
        }
        "claude_context" => "Context window usage for Claude Code session",
        "claude_cost" => "Cost info for Claude Code session",
        "claude_model" => "AI model name for Claude Code session",
        "cmake" => "The currently installed version of CMake",
        "cmd_duration" => "How long the last command took to execute",
        "cobol" => "The currently installed version of COBOL/GNUCOBOL",
        "conda" => "The current conda environment, if $CONDA_DEFAULT_ENV is set",
        "container" => "The container indicator, if inside a container.",
        "cpp" => "your cpp compiler type",
        "crystal" => "The currently installed version of Crystal",
        "daml" => "The Daml SDK version of your project",
        "dart" => "The currently installed version of Dart",
        "deno" => "The currently installed version of Deno",
        "directory" => "The current working directory",
        "direnv" => "The currently applied direnv file",
        "docker_context" => "The current docker context",
        "dotnet" => "The relevant version of the .NET Core SDK for the current directory",
        "elixir" => "The currently installed versions of Elixir and OTP",
        "elm" => "The currently installed version of Elm",
        "erlang" => "Current OTP version",
        "fennel" => "The currently installed version of Fennel",
        "fill" => "Fills the remaining space on the line with a pad string",
        "fortran" => "The currently used version of Fortran",
        "fossil_branch" => "The active branch of the check-out in your current directory",
        "fossil_metrics" => "The currently added/deleted lines in your check-out",
        "gcloud" => "The current GCP client configuration",
        "git_branch" => "The active branch of the current Git repo",
        "git_commit" => "The active commit (and tag if any) of the current Git repo",
        "git_metrics" => "The currently added/deleted lines in your Git repo",
        "git_state" => "The current Git operation, and it's progress",
        "git_status" => {
            "Symbols representing the state of the current Git repo, filtered to your current directory"
        }
        "gleam" => "The currently installed version of Gleam",
        "golang" => "The currently installed version of Golang",
        "gradle" => "The currently installed version of Gradle",
        "guix_shell" => "The guix-shell environment",
        "haskell" => "The selected version of the Haskell toolchain",
        "haxe" => "The currently installed version of Haxe",
        "helm" => "The currently installed version of Helm",
        "hg_branch" => "The active branch and topic of the repo in your current directory",
        "hg_state" => "The current hg operation",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "julia" => "The currently installed version of Julia",
        "kotlin" => "The currently installed version of Kotlin",
        "kubernetes" => "The current Kubernetes context name and, if set, the namespace",
        "line_break" => "Separates the prompt into two lines",
        "localip" => "The currently assigned ipv4 address",
        "lua" => "The currently installed version of Lua",
        "maven" => "The Maven Wrapper version of the current project",
        "memory_usage" => "Current system memory and swap usage",
        "meson" => {
            "The current Meson environment, if $MESON_DEVENV and $MESON_PROJECT_NAME are set"
        }
        "mise" => "The current mise status",
        "mojo" => "The currently installed version of Mojo",
        "nats" => "The current NATS context",
        "netns" => "The current network namespace",
        "nim" => "The currently installed version of Nim",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of NodeJS",
        "ocaml" => "The currently installed version of OCaml",
        "odin" => "The currently installed version of Odin",
        "opa" => "The currently installed version of Open Platform Agent",
        "openstack" => "The current OpenStack cloud and project",
        "os" => "The current operating system",
        "package" => "The package version of the current directory's project",
        "perl" => "The currently installed version of Perl",
        "php" => "The currently installed version of PHP",
        "pijul_channel" => "The current channel of the repo in the current directory",
        "pixi" => {
            "The currently installed version of Pixi, and the active environment if $PIXI_ENVIRONMENT_NAME is set"
        }
        "pulumi" => "The current username, stack, and installed version of Pulumi",
        "purescript" => "The currently installed version of PureScript",
        "python" => "The currently installed version of Python",
        "quarto" => "The current installed version of quarto",
        "raku" => "The currently installed version of Raku",
        "red" => "The currently installed version of Red",
        "rlang" => "The currently installed version of R",
        "ruby" => "The currently installed version of Ruby",
        "rust" => "The currently installed version of Rust",
        "scala" => "The currently installed version of Scala",
        "shell" => "The currently used shell indicator",
        "shlvl" => "The current value of SHLVL",
        "singularity" => "The currently used Singularity image",
        "solidity" => "The current installed version of Solidity",
        "spack" => "The current spack environment, if $SPACK_ENV is set",
        "status" => "The status of the last command",
        "sudo" => "The sudo credentials are currently cached",
        "swift" => "The currently installed version of Swift",
        "terraform" => "The currently selected terraform workspace and version",
        "time" => "The current local time",
        "typst" => "The current installed version of typst",
        "username" => "The active user's username",
        "vagrant" => "The currently installed version of Vagrant",
        "vcs" => "The currently active VCS repository (first one matching)",
        "vcsh" => "The currently active VCSH repository",
        "vlang" => "The currently installed version of V",
        "xmake" => "The currently installed version of XMake",
        "zig" => "The currently installed version of Zig",
        _ => "<no description>",
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::module::ALL_MODULES;

    #[test]
    fn all_modules_have_description() {
        for module in ALL_MODULES {
            println!("Checking if {module:?} has a description");
            assert_ne!(description(module), "<no description>");
        }
    }
}
