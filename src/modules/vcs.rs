
use super::{Context, Module, ModuleConfig};

use crate::configs::vcs::VcsConfig;
use crate::formatter::StringFormatter;
use crate::formatter::string_formatter::StringFormatterError;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vcs");
    let config = VcsConfig::try_load(module.config);

    if config.disabled || config.order.is_empty() {
        return None;
    }

    if !config
        .order
        .into_iter()
        .any(|vcs| vcs == "git" && context.get_git_repo().is_ok())
    {
        return None;
    }

    let modules = config.git_modules;

    if modules.is_empty() {
        return None;
    }

    let parsed = StringFormatter::new(modules).and_then(|formatter| {
        formatter
            .map_variables_to_segments(|variable| match variable {
                "vcs" => Some(Err(StringFormatterError::Custom(
                    "cannot recursively include the `vcs` module in itself".into(),
                ))),
                module => super::handle(module, context).map(|m| Ok(m.segments)),
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vcs`:\n{error}");
            return None;
        }
    });

    Some(module)
}


#[cfg(test)]
mod tests {
    use std::io;

    use nu_ansi_term::Color;

    use crate::test::{COMMON_GIT_PROVIDERS, ModuleRenderer, fixture_repo};

    #[test]
    fn empty_order_disables() {
        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = []
            })
            .collect();
        assert_eq!(actual, None);
    }

    #[test]
    fn empty_modules_disables() -> io::Result<()> {
        let repo_dir = fixture_repo(COMMON_GIT_PROVIDERS[0])?;
        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = ["git"]
                git_modules = ""
            })
            .path(repo_dir.path())
            .collect();
        assert_eq!(actual, None);
        repo_dir.close()
    }

    #[test]
    fn recursive_vcs_include_fails() -> io::Result<()> {
        let repo_dir = fixture_repo(COMMON_GIT_PROVIDERS[0])?;
        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = ["git"]
                git_modules = "$vcs"
            })
            .path(repo_dir.path())
            .collect();
        assert_eq!(actual, None);
        repo_dir.close()
    }

    #[test]
    fn detect_git() -> io::Result<()> {
        for &mode in COMMON_GIT_PROVIDERS {
            let repo_dir = fixture_repo(mode)?;
            let actual = ModuleRenderer::new("vcs")
                .config(toml::toml! {
                    [vcs]
                    order = ["git"]
                    git_modules = "${custom.test}"
                    [custom.test]
                    command = "echo test"
                    when = true
                })
                .path(repo_dir.path())
                .collect();
            let expected = Some(format!(
                "{}",
                Color::Green.bold().paint("test ")
            ));
            assert_eq!(actual, expected);
            repo_dir.close()?;
        }
        Ok(())
    }

    #[test]
    fn invalid_vcs_is_none() -> io::Result<()> {
        let repo_dir = fixture_repo(COMMON_GIT_PROVIDERS[0])?;
        let actual = ModuleRenderer::new("vcs")
            .config(toml::toml! {
                [vcs]
                order = ["does_not_exist"]
            })
            .path(repo_dir.path())
            .collect();
        assert_eq!(actual, None);
        repo_dir.close()
    }
}
