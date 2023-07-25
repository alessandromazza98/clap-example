use api::stringer::reverse;
use clap::{arg, command, Args, Command, Parser, Subcommand};

use crate::api::stringer::inspect;

mod api;

#[derive(Parser)]
#[command(author, version)]
#[command(
    about = "stringer - a simple CLI to transform and inspect strings",
    long_about = "stringer is a super fancy CLI (kidding)

One can use stringer to modify or inspect strings straight from the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Reverses a string
    Reverse(Reverse),
    /// Inspects a string
    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    /// The string to reverse
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    /// The string to inspect
    string: Option<String>,
    #[arg(short = 'd', long = "digits")]
    only_digits: bool,
}

fn main() {
    // one() and two() functions are inter-changeable.
    // they do the same things.
    one()
}

fn one() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Reverse(name)) => match name.string {
            Some(ref _name) => {
                let reverse = reverse(_name);
                println!("{}", reverse);
            }
            None => {
                println!("Please provide a string to reverse");
            }
        },
        Some(Commands::Inspect(name)) => match name.string {
            Some(ref _name) => {
                let (res, kind) = inspect(_name, name.only_digits);

                let mut plural_s = "s";
                if res == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", _name, res, kind, plural_s);
            }
            None => {
                println!("Please provide a string to inspect");
            }
        },
        None => {}
    }
}

fn two() {
    let matches = command!()
        .about("stringer - a simple CLI to transform and inspect strings")
        .long_about(
            "stringer is a super fancy CLI (kidding)

        One can use stringer to modify or inspect strings straight from the terminal",
        )
        .subcommand(
            Command::new("reverse")
                .about("Reverses a string")
                .arg(arg!([STRING] "The string to reverse")),
        )
        .subcommand(
            Command::new("inspect")
                .about("Inspects a string")
                .arg(arg!([STRING] "The string to inspect"))
                .arg(arg!(-d --digits "only digits")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("reverse") {
        let str_to_reverse = matches.get_one::<String>("STRING");
        match str_to_reverse {
            Some(string) => println!("{}", reverse(string)),
            None => println!("Please provide a string to reverse"),
        };
    }

    if let Some(matches) = matches.subcommand_matches("inspect") {
        let str_to_inspect = matches.get_one::<String>("STRING");
        match str_to_inspect {
            Some(string) => {
                let only_digits = matches.get_flag("digits");
                let (res, kind) = inspect(string, only_digits);

                let mut plural_s = "s";
                if res == 1 {
                    plural_s = "";
                }

                println!("{:?} has {} {}{}.", string, res, kind, plural_s);
            }
            None => println!("Please provide a string to inspect"),
        };
    }
}
