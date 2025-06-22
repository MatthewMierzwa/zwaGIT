use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

/// zwagit: a mini VCS
#[derive(Parser)]
#[command(name = "zwagit")]
#[command(about = "A mini Git-like version control system written in Rust.")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Init a new zwagit repo
    Init,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            init_repo();
        }
    }
}

/// Creates a .zwagit directory with objects/ and refs/
fn init_repo() {
    let zwagit_path = Path::new(".zwagit");
    
    if zwagit_path.exists() {
        println!("Repository already initialized.");
        return;
    }

    fs::create_dir_all(zwagit_path.join("objects")).expect("Failed to create objects directory");
    fs::create_dir_all(zwagit_path.join("refs")).expect("Failed to create refs directory");

    println!("Initialized empty zwagit repository in .zwagit/");
}
