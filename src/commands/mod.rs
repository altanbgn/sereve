use clap::Subcommand;

pub mod add;
pub mod remove;
pub mod list;

#[derive(Subcommand, Debug)]
pub enum MainCommand {
    #[command(about = "Add a new server")]
    Add(add::Add),

    #[command(about = "Remove a server")]
    Remove(remove::Remove),

    #[command(about = "Lists servers")]
    List(list::List),
}

