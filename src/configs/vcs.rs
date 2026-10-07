use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcsConfig<'a> {
    /// Order in which to discover VCSes.
    pub order: Vec<&'a str>,
    /// Disables the VCS module.
    pub disabled: bool,
    /// Modules to use when Git is matched.
    pub git_modules: &'a str,
}

impl Default for VcsConfig<'_> {
    fn default() -> Self {
        Self {
            order: vec!["git"],
            disabled: false,
            git_modules: "$git_branch$git_commit$git_state$git_metrics$git_status",
        }
    }
}
