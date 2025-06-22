use clap::{Parser, Subcommand};
use std::path::Path;

use std::fs::{self, File};
use std::io::{Read, Write};
use sha1::{Sha1, Digest};

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

    HashObject {
        /// Path to the file to hash
        path: String,

        /// Write the object to the object store
        #[arg(short = 'w', long = "write")]
        write: bool,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_repo(),
        Commands::HashObject { path, write } => hash_object(&path, write),
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

fn hash_object(path: &str, write: bool) {
    let mut file = File::open(path).expect("Failed to open file");
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect("Failed to read file");

    // Create a block header: "blob <len>\0"
    let header = format!("blob {}\0", content.len());
    let mut store = Vec::new();
    store.extend_from_slice(header.as_bytes());
    store.extend_from_slice(&content);

    let mut hasher = Sha1::new();
    hasher.update(&store);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    println!("{}", hash_hex);

    if write {
        // Save to .zwagit/objects/xx/yyyy...
        let (dir, file) = hash_hex.split_at(2);
        let object_dir = format!(".zwagit/objects/{}", dir);
        let object_path = format!("{}/{}", object_dir, file);

        fs::create_dir_all(&object_dir).expect("Failed to create object directory");
        let mut f = File::create(&object_path).expect("Failed to create object file");
        f.write_all(&store).expect("Failed to write object file");
    }

}
