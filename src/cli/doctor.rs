use color_eyre::eyre::{eyre, Result};
use console::style;
use indoc::formatdoc;
use once_cell::sync::Lazy;

use crate::cli;
use crate::cli::command::Command;
use crate::config::Config;
use crate::env;

use crate::output::Output;

/// Check rtx installation for possible problems.
#[derive(Debug, clap::Args)]
#[clap(verbatim_doc_comment, after_long_help = AFTER_LONG_HELP.as_str())]
pub struct Doctor {}

impl Command for Doctor {
    fn run(self, config: Config, out: &mut Output) -> Result<()> {
        let mut checks = Vec::new();
        for plugin in config.plugins.values() {
            if !plugin.is_installed() {
                checks.push(format!("plugin {} is not installed", plugin.name));
                continue;
            }
        }

        if let Some(latest) = cli::version::check_for_new_version() {
            warn!(
                "new rtx version {} available, currently on {}",
                latest,
                env!("CARGO_PKG_VERSION")
            )
        }

        rtxprintln!(out, "{}\n", &config);
        rtxprintln!(out, "{}\n", rtx_env_vars());

        if !config.is_activated() {
            checks.push(
                "rtx is not activated, run `rtx help activate` for setup instructions".to_string(),
            );
        }

        for check in &checks {
            error!("{}", check);
        }

        if checks.is_empty() {
            rtxprintln!(out, "No problems found");
            Ok(())
        } else {
            Err(eyre!("{} problems found", checks.len()))
        }
    }
}

fn rtx_env_vars() -> String {
    let vars = env::vars()
        .filter(|(k, _)| k.starts_with("RTX_"))
        .collect::<Vec<(String, String)>>();
    let mut s = style("rtx environment variables:\n").bold().to_string();
    for (k, v) in vars {
        s.push_str(&format!("{}={}\n", k, v));
    }
    s
}

static AFTER_LONG_HELP: Lazy<String> = Lazy::new(|| {
    formatdoc! {r#"
    {}
      $ rtx doctor
      [WARN] plugin nodejs is not installed
    "#, style("Examples:").bold().underlined()}
});

#[cfg(test)]
mod tests {
    use crate::cli::tests::cli_run;

    #[test]
    fn test_doctor() {
        let _ = cli_run(
            &vec!["rtx", "doctor"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<String>>(),
        );
    }
}
