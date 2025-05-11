use clap::Parser;

#[derive(Parser)]
#[command(version, about = "")]
pub struct Arguments {
    #[arg(short, long)]
    pub savefile: String,

    #[arg(short, long)]
    pub encode: bool
}
