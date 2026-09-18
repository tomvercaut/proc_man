use clap::{Parser, Subcommand};
use proc_man::{db, default_db_path};
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[command(propagate_version = true)]
enum Commands {
    /// Start a specific process by name
    Start {
        #[arg(help = "Name")]
        name: String,
    },
    /// Start all the processes
    StartAll,
    /// Stop a specific process by name
    Stop {
        #[arg(help = "Name")]
        name: String,
    },
    /// Stop all the processes
    StopAll,
    /// Restart a specific process by name
    Restart {
        #[arg(help = "Name")]
        name: String,
    },
    /// Restart all the processes
    RestartAll,
    Db {
        #[command(subcommand)]
        db: DbCommands,
    },
}

/// Manage all the configuration of the processes
#[derive(Subcommand, Debug)]
enum DbCommands {
    /// Initialise a new database in which all the processes to launch are stored.
    Init,
    /// Add a new process to the database
    Add,
    /// List all the processes in the database
    List,
    /// Remove a process from the database
    Remove {
        #[arg(help = "Name")]
        name: String,
    },
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_ansi(false)
        .init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { name } => {
            debug!("DUMMY Starting process {}", name);
        }
        Commands::StartAll => {
            debug!("DUMMY Starting all processes");
        }
        Commands::Stop { name } => {
            debug!("DUMMY Stopping process {}", name);
        }
        Commands::StopAll => {
            debug!("DUMMY Stopping all processes");
        }
        Commands::Db { db } => {
            debug!("DUMMY Managing database");
            match db {
                DbCommands::Init => {
                    let db_path = default_db_path()?;
                    let parent = db_path.parent().unwrap();
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)?;
                    }
                    db::init(&db_path)?;
                }
                DbCommands::Add => {
                    debug!("DUMMY Adding process to database");
                }
                DbCommands::List => {
                    debug!("DUMMY Listing processes in database");
                }
                DbCommands::Remove { name } => {
                    debug!("DUMMY Removing process ({}) from database", name);
                }
            }
        }
        Commands::Restart { name } => {
            debug!("DUMMY Restarting process {}", name);
        }
        Commands::RestartAll => {
            debug!("DUMMY Restarting all processes");
        }
    }

    Ok(())
}
