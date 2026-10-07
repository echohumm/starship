use std::path::{Path, PathBuf};
use std::{env, io};

use which::which;

/* Fish uses a small init stub that sources the full generated script. */

struct StarshipPath {
    native_path: PathBuf,
}
impl StarshipPath {
    fn init() -> io::Result<Self> {
        let exe_name = option_env!("CARGO_PKG_NAME").unwrap_or("starship");
        let native_path = which(exe_name).or_else(|_| env::current_exe())?;
        Ok(Self { native_path })
    }

    fn sprint(&self) -> io::Result<String> {
        self.native_path
            .to_str()
            .ok_or_else(|| io::Error::other("can't convert to str"))
            .map(|p| shell_words::quote(p).into_owned())
    }
}

/* This prints the setup stub, the short piece of code which sets up the main
init code. The stub produces the main init script, then evaluates it with
`source` and process substitution */
pub fn init_stub(shell_name: &str) -> io::Result<()> {
    let shell_basename = Path::new(shell_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(shell_name);

    if shell_basename != "fish" {
        eprintln!("{shell_basename} is not supported by this build; only fish is enabled.");
        return Ok(());
    }

    let starship = StarshipPath::init()?;
    print!(r"source ({} init fish --print-full-init | psub)", starship.sprint()?);
    Ok(())
}

pub fn init_main(shell_name: &str) -> io::Result<()> {
    if shell_name != "fish" {
        println!("printf \"Only fish initialization is enabled in this build.\n\"");
        return Ok(());
    }

    let starship_path = StarshipPath::init()?;
    print_script(FISH_INIT, &starship_path.sprint()?);
    Ok(())
}

fn print_script(script: &str, path: &str) {
    let script = script.replace("::STARSHIP::", path);
    print!("{script}");
}

const FISH_INIT: &str = include_str!("starship.fish");

