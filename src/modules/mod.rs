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
mod vcsh;
mod zig;

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
            "username" => username::module(context),
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
        "c" => "Your C compiler type",
        "character" => "A character beside where text is entered",
        "cmake" => "The currently installed version of CMake",
        "cmd_duration" => "How long the last command took to execute",
        "container" => "The container indicator, if inside a container",
        "cpp" => "Your C++ compiler type",
        "directory" => "The current working directory",
        "direnv" => "The currently applied direnv file",
        "docker_context" => "The current Docker context",
        "dotnet" => "The relevant .NET SDK version",
        "fill" => "Fills the remaining terminal width with a pad string",
        "git_branch" => "The active branch of the current Git repository",
        "git_commit" => "The active commit and tag of the current Git repository",
        "git_metrics" => "The currently added/deleted lines in the Git repository",
        "git_state" => "The current Git operation and its progress",
        "git_status" => "Symbols representing the state of the current Git repository",
        "golang" => "The currently installed version of Go",
        "gradle" => "The currently installed version of Gradle",
        "hostname" => "The system hostname",
        "java" => "The currently installed version of Java",
        "jobs" => "The current number of jobs running",
        "kotlin" => "The currently installed version of Kotlin",
        "line_break" => "Separates the prompt into two lines",
        "maven" => "The Maven Wrapper version of the current project",
        "meson" => "The current Meson environment",
        "nix_shell" => "The nix-shell environment",
        "nodejs" => "The currently installed version of Node.js",
        "package" => "The package version of the current directory's project",
        "python" => "The currently installed version of Python",
        "rust" => "The currently installed version of Rust",
        "shell" => "The currently used shell indicator",
        "shlvl" => "The current value of SHLVL",
        "status" => "The status of the last command",
        "sudo" => "Whether sudo credentials are currently cached",
        "time" => "The current local time",
        "username" => "The active user's username",
        "vcsh" => "The currently active VCSH repository",
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
