mod config;
mod logger;
mod profile;
mod slurm;

use std::{fs, path::PathBuf, process::Command};

use anyhow::{Context, Result};
use clap::{AppSettings, Parser};
use log::{debug, info, warn};

use config::{get_config_path, Config};
use profile::Profile;

#[derive(Debug, Parser)]
#[clap(version, about)]
#[clap(global_setting(AppSettings::ArgRequiredElseHelp))]
#[clap(global_setting(AppSettings::HelpExpected))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Cli {
    /// Custom config file location
    #[clap(long, value_name = "PATH")]
    config: Option<PathBuf>,
    /// Profile name to use
    #[clap(long, value_name = "NAME")]
    profile: Option<String>,
    /// Export to sbatch script instead of executing srun command
    #[clap(long, value_name = "PATH")]
    export: Option<PathBuf>,
    #[clap(flatten)]
    args: Profile,
    /// DO NOT execute; just print the command/script on stdout
    #[clap(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    logger::init().context("cannot initialize logger")?;

    let cli = Cli::parse();
    debug!("command line arguments:\n{:?}", cli);

    let filepath = get_config_path(cli.config).context("cannot find the config file path")?;
    info!("read config from {}", filepath.as_path().display());

    let content = fs::read_to_string(filepath).context("cannot read the content of config file")?;
    let mut config: Config = toml::from_str(&content).context("cannot parse the content")?;
    debug!("config:\n{:?}", config);

    let profile_name = cli
        .profile
        .or({
            warn!("use default profile name in config file");
            config.default
        })
        .context("cannot determine which profile to use")?;
    info!("profile name: {}", profile_name);

    let profile = config
        .profile
        .remove(&profile_name)
        .context(format!("the profile {} does not exist", profile_name))?
        .overwrite(cli.args);
    debug!("profile:\n{}", toml::to_string(&profile)?);

    match cli.export {
        Some(scriptfile) => {
            let script = slurm::sbatch(&profile).context("cannot generate sbatch script")?;
            if cli.dry_run || cfg!(debug_assertions) {
                info!("generated sbatch script:");
                println!("{}", script);
            } else {
                fs::write(&scriptfile, script).context(format!(
                    "cannot write sbatch script to {}",
                    scriptfile.as_path().display()
                ))?;
                info!("sbatch script at {}", scriptfile.as_path().display());
                let mut process = Command::new("sbatch")
                    .arg(scriptfile.as_path())
                    .spawn()
                    .context("failed to execute sbatch")?;
                process.wait().context("sbatch wasn't running")?;
            }
        }
        None => {
            let arguments = slurm::srun(&profile).context("cannot generate srun arguments")?;
            if cli.dry_run || cfg!(debug_assertions) {
                info!("generated srun command:");
                println!("srun {}", arguments.join(" "));
            } else {
                info!("executing srun");
                let mut process = Command::new("srun")
                    .args(arguments)
                    .spawn()
                    .context("failed to execute srun")?;
                process.wait().context("srun wasn't running")?;
            }
        }
    };
    Ok(())
}
