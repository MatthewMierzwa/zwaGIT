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
    },
    CatFile {
        /// Print the contents of the object
        #[arg(short = 'p', long = "pretty")]
        pretty: bool,

        /// The SHA-1 hash of the object
        hash: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_repo(),
        Commands::HashObject { path, write } => hash_object(&path, write),
        Commands::CatFile { pretty, hash } => cat_file(&hash, pretty),
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

fn cat_file(hash: &str, pretty: bool) {
    if hash.len() != 40 {
        eprintln!("Invalid hash length; must be 40 hex characters.");
        return;
    }

    let (dir, file) = hash.split_at(2);
    let object_path = format!(".zwagit/objects/{}/{}", dir, file);

    let data = match fs::read(&object_path) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Object not found: {}", hash);
            return;
        }
    };

    if pretty {
        // Format: "blob <len>\0<content>"
        // Find the first null byte, then print everything after it
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let content = &data[null_pos + 1..];
            match std::str::from_utf8(content) {
                Ok(text) => println!("{}", text),
                Err(_) => eprintln!("Failed to parse object content as UTF-8.");
            }
        } else {
            eprintln!("Malformed object: no null byte found.");
        }
    } else {
        // Print raw data (might include header + content)
        match std::str::from_utf8(&data) {
            Ok(text) => println!("{}", text),
            Err(_) => eprintln!("Failed to parse object as UTF-8."),
        }
    }
}