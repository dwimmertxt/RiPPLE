use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(
    name="RiPPLE", author="dwimmer", version="0.1.3", 
    about="A multi-sensory signal hunting operation.", 
    arg_required_else_help=true, hide_possible_values=true)]
pub struct Prism {
    #[clap(display_order=1, short='r', long="resolution",
        value_parser=clap::value_parser!(i32), default_value="1")]
    pub resolution: i32,

    #[clap(display_order=2, short='w', long="width",
        value_parser=clap::value_parser!(i32), default_value="32")]
    pub width: i32,

    #[clap(display_order=3, short='h', long="height",
        value_parser=clap::value_parser!(i32), default_value="32")]
    pub height: i32,

    #[clap(display_order=4, short='n', long="normalise", 
        value_parser=clap::value_parser!(i32).range(0..), 
        use_delimiter=true, number_of_values=2)]
    pub norm: Vec<i32>,

    #[clap(display_order=5, short='f', long="fft")]
    pub fft: bool,

    #[clap(display_order=6, short='H', long="harmonics")]
    pub harmonics: bool,

    #[clap(display_order=7, short='j', long="jsonl")]
    pub jsonl: bool,

    #[clap(display_order=8, short='p', long="pipe", 
        value_parser=["t", "f", "h"])] // max_value = 1
    pub pipe:           Vec<String>,
    #[clap(subcommand)]
    command: Commands,
}

    