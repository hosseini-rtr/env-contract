mod contract;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "env-contract",
    version,
    about = "Manage and validate environment variable contracts"
)]
struct Cli {
    #[command(flatten)]
    global: GlobalArgs,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Args)]
struct GlobalArgs {
    /// Path to the environment contract
    #[arg(
        short = 'c',
        long,
        default_value = "./env.contract.yaml",
        global = true
    )]
    contract: String,

    /// Limit the command to one target environment file
    #[arg(short = 't', long, global = true)]
    target: Option<String>,

    /// Run without interactive prompts
    #[arg(long, global = true)]
    ci: bool,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Create environment files from a contract or infer a contract
    Init,
    /// Synchronize environment files with a contract
    Sync,
    /// Validate environment files against a contract
    Check,
    // TODO: Add variable to target file
    // Add,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), contract::ContractError> {
    if matches!(cli.command, Command::Check) {
        let loaded = contract::Contract::load(&cli.global.contract)?;
        println!(
            "loaded contract {} ({} variable(s), {} target file(s))",
            cli.global.contract,
            loaded.variables.len(),
            loaded.target_files().len()
        );
    }

    if !matches!(cli.command, Command::Check) {
        let action = match cli.command {
            Command::Init => "init",
            Command::Sync => "sync",
            Command::Check => unreachable!(),
        };
        let target = cli.global.target.as_deref().unwrap_or("all targets");
        let mode = if cli.global.ci { "CI" } else { "interactive" };

        println!(
            "{action} command selected (contract: {}, target: {target}, mode: {mode})",
            cli.global.contract
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_check_with_global_options() {
        let cli = Cli::try_parse_from([
            "env-contract",
            "check",
            "--contract",
            "config/contract.yaml",
            "--target",
            "db.env",
            "--ci",
        ])
        .expect("valid CLI arguments");

        assert!(matches!(cli.command, Command::Check));
        assert_eq!(cli.global.contract, "config/contract.yaml");
        assert_eq!(cli.global.target.as_deref(), Some("db.env"));
        assert!(cli.global.ci);
    }

    #[test]
    fn uses_default_contract_and_interactive_mode() {
        let cli = Cli::try_parse_from(["env-contract", "init"]).expect("valid CLI arguments");

        assert!(matches!(cli.command, Command::Init));
        assert_eq!(cli.global.contract, "./env.contract.yaml");
        assert!(cli.global.target.is_none());
        assert!(!cli.global.ci);
    }
}
