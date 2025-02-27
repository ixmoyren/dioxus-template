#![feature(future_join)]
use crate::task::{build, build_tailwind, clean, fmt, run_dx_serve};
use clap::{Parser, Subcommand};

mod task;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    #[command(about = "Build the project")]
    Build {
        #[arg(short = 'c', help = "Cargo build")]
        cargo: bool,
        #[arg(short = 'd', help = "Dioxus build")]
        dioxus: bool,
        #[arg(short = 'r', help = "release")]
        release: bool,
    },
    #[command(about = "Build the Tailwindcss")]
    BuildTailwind {
        #[arg(short = 'i', long, help = "Installation dependency")]
        install: bool,
        #[arg(short = 'w', long, help = "Enable watching")]
        watch: bool,
    },
    #[command(about = "Run dioxus serve")]
    RunDxServe,
    #[command(about = "Clean build")]
    Clean,
    #[command(about = "Formatted code")]
    Fmt,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let _ = async_io::block_on(async {
        use crate::Action::*;
        match cli.action {
            Build {
                cargo,
                dioxus,
                release,
            } => build(cargo, dioxus, release).await,
            BuildTailwind { install, watch } => build_tailwind(install, watch).await,
            RunDxServe => run_dx_serve().await,
            Clean => clean().await,
            Fmt => fmt().await,
        }
    });

    Ok(())
}
