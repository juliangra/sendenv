use arboard::Clipboard;
use clap::Parser;
use std::process;
use std::{
    fs::{read_to_string, File},
    process::Command,
};

#[derive(Parser, Debug)]
#[command(
    about = "Uploads a file to 1Password as a secure note and shares it. The share link will be copied to your clipboard or sent to the email recipient of choice if the `--email` flag is set."
)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = ".env",
        value_name = "PATH_TO_ENV",
        help = "Path of the file you want to share"
    )]
    path: String,

    #[arg(
        short,
        long,
        value_name = "EMAIL ADDRESS",
        help = "[OPTIONAL] User of link will have to enter their email address to view the link"
    )]
    email: Option<String>,

    #[arg(
        short,
        long,
        help = "[OPTIONAL] Makes the link expire after a single use"
    )]
    display_once: bool,
}

fn main() {
    // Parse arguments and initialize shared variables
    let args = Args::parse();
    let devnull = File::open("/dev/null").unwrap();

    // Attempt to read the contents of the provided file from `args.path`
    let env_contents = read_to_string(&args.path).unwrap_or_else(|_| {
        println!("❓File with name `{}` not found", args.path);
        process::exit(0)
    });

    // Check whether the user has a valid 1Password session or not by listing users
    let signed_in = Command::new("op")
        .arg("user")
        .arg("list")
        .stdout(devnull.try_clone().unwrap())
        .stderr(devnull)
        .status()
        .expect("Unable to check authentication status for 1Password.");

    // If there is no active session, prompt the user to login before using the application
    if !signed_in.success() {
        println!(
            "❌ You are currently signed out of 1Password. Please sign in to continue.\n   To sign in, run `eval $(op signin)` before running this command"
        );
        process::exit(0)
    }

    // Execute the `op item create` command to create a new secure note
    let create_item = Command::new("op")
        .arg("item")
        .arg("create")
        .arg("--category=secure note")
        .arg(format!("--title={}", args.path))
        .arg(format!("notesPlain={}", env_contents))
        .output()
        .expect("❌ Failed to create 1Password entry");

    // Convert the result of the creation to a string
    let result =
        String::from_utf8(create_item.stdout).expect("❌ Failed to create 1Password entry");

    // Extract the id from the newly created secure note
    let id_prefix = "ID: ";
    let id_start = result
        .find(id_prefix)
        .expect("❌ Failed to create 1Password entry")
        + id_prefix.len();
    let id_end = result[id_start..]
        .find('\n')
        .unwrap_or(result.len() - id_start);
    let item_id = &result[id_start..id_start + id_end].trim();

    // Initialize a new instance of the 1Password CLI and append
    // the item_id extracted from the new secure note
    let mut share_cmd = Command::new("op");
    share_cmd.arg("item").arg("share").arg(item_id);

    // If the `--display-once` flag is set, add the `--view-once` argument
    if args.display_once {
        println!("\n🔒 Share link will expire after a single use");
        share_cmd.arg("--view-once");
    }

    // If the `--email` flag is set, add it to the command
    if let Some(email) = &args.email {
        println!("📧 Sharing with email address: {}", email);
        share_cmd.arg(format!("--emails={}", email));
    }

    // Retrieve the result of the `op share item` command and convert it to a string
    let share_item = share_cmd.output().expect("❌ Failed to create share link");
    let share_item_string =
        String::from_utf8(share_item.stdout).expect("❌ Failed to create share link");

    // Trim the excess quotes and newlines from the share-link and
    // display it to the user
    let share_link = share_item_string.trim_matches('"').trim();
    println!("🔗 Share link: {}", share_link);

    // Initialize a new Clipboard modifier instance and set
    // the text contents to the share-link
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(share_link).unwrap();

    println!( "\n✅ `{}` has been shared successfully! The share link has been copied to your clipboard 📋", args.path);
}
