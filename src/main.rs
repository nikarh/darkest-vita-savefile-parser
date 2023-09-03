use clap::{Parser, Subcommand};
use decoder::decode;
use encoder::encode;

mod decoder;
mod encoder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Decode a Darkest Dungeon save file profile_N.z from Vita into a directory with json files.
    /// Keep in mind that save files are encrypted on Vita, so it must be decrypted first (e.g. in VitaShell)
    Decode {
        /// A path to a .z save file
        #[arg(short, long)]
        input_file: String,
        /// Target directory. Json files will be saved in output_dir/profile_N directory
        #[arg(short, long)]
        output_dir: String,
    },
    /// Encode a folder with Darkest Dungeon Json save files into a .z save file for Vita.
    /// Keep in mind that save files are encrypted on Vita, and this tool will NOT encrypt your save file.
    Encode {
        /// A path to a profile_N directory with Json files.
        #[arg(short, long)]
        input_dir: String,
        /// A path to a target .z file.
        /// The profile name (e.g. profile_0) is embedded in the save file, and is taken from output_file name by default,
        /// so if you name it profile_1.z, it will embed profile_1 in the file. If you later rename the file, the save will not
        /// work on vita.
        #[arg(short, long)]
        output_file: String,
        /// If provided, overrides the profile name embedded in the save file. By default profile name will be the last segment
        /// of the output_file path without the extension.
        #[arg(short, long)]
        profile: Option<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Decode {
            input_file,
            output_dir,
        } => decode(&input_file, &output_dir),
        Commands::Encode {
            input_dir,
            output_file,
            profile,
        } => encode(&input_dir, &output_file, profile.as_deref()),
    }
}
