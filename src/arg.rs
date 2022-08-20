use clap::{Parser};


#[derive(Debug, Parser)]
#[clap(
    name="RiPPLE", author="dwimmer", version="0.1.2", 
    about="A multi-sensory signal hunting operation.", 
    arg_required_else_help=true, hide_possible_values=true)]
pub struct RippleArgs {
    #[clap(display_order=3, short='r', long="resolution",
        value_parser=clap::value_parser!(i32), default_value="44100")]
    pub resolution:    i32,
    #[clap(display_order=2, short='n', long="normalise", 
        value_parser=clap::value_parser!(i32).range(0..), 
        use_delimiter=true, number_of_values=2)]
    pub norm:           Vec<i32>,
    #[clap(display_order=1, short='f', long="fft")]
    pub fft:            bool,
    #[clap(display_order=4, short='W', long="wav")]
    pub wav:            bool,
    #[clap(display_order=5, short='C', long="csv", 
        value_parser=["t","f"], use_delimiter=true, max_values=2)]
    pub csv:            Vec<String>,
    #[clap(display_order=6, short='P', long="pipe", 
        value_parser=["t", "f"])]
    pub pipe:           Vec<String>,
    #[clap(display_order=7, short='b', long="bytes")]
    pub bytes:          bool,
}


    