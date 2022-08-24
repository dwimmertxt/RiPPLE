use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(
    name="RiPPLE", author="dwimmer", version="0.1.4", 
    about="A multi-sensory signal hunting operation.", 
    arg_required_else_help=true, hide_possible_values=true)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Convert data provided via stdin or file to ripl or jsonl and then pipe to stdout. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(long_flag="Convert", name="C", author="dwimmer", disable_help_flag=true)]
    Convert {
        /// convert to ripl format
        #[clap(display_order=1, short='r', long="ripl")]
        ripl: bool,
        /// convert to jsonl format
        #[clap(display_order=2, short='j', long="jsonl")]
        jsonl: bool,
        /// save result to file
        #[clap(display_order=3, short='s', long="save")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=4, short='v', long="valve")]
        valve: bool,
        /// specify input as time domain
        #[clap(display_order=5, short='t', long="time", conflicts_with_all=&["freq", "harm"])]
        time: bool,
        /// specify input as frequency domain
        #[clap(display_order=6, short='f', long="freq", conflicts_with_all=&["time", "harm"])]
        freq: bool,
        /// specify input as harmonic domain
        #[clap(display_order=7, short='h', long="harm", conflicts_with_all=&["time", "freq"])]
        harm: bool,
        /// name of file to import
        #[clap(display_order=8, value_parser)]
        file: Vec<String>,
    },
    /// Generate a time domain from the output of the RiPPLE function and pipe to stdout. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(long_flag="Ripple", name="R", author="dwimmer", disable_help_flag=true)]
    Ripple {
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=1, short='w', long="width", 
            value_parser=clap::value_parser!(u16), default_value="64")]
        width: u16,
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=2, short='h', long="height", 
            value_parser=clap::value_parser!(u16), default_value="64")]
        height: u16,
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=3, short='d', long="depth", 
            value_parser=clap::value_parser!(u16), default_value="64")]
        depth: u16,
        /// time resolution of ripple; total number of frames
        #[clap(display_order=4, short='r', long="resolution",
            value_parser=clap::value_parser!(u16), default_value="1")]
        resolution: u16,
        /// save result to file
        #[clap(display_order=5, short='s', long="save")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=6, short='v', long="valve")]
        valve: bool,
    },
    /// Generate a frequency domain by performing a Fast Fourier Transform 
    /// on the input time domain and either save to file or pipe to stdout. 
    #[clap(long_flag="Frequency", name="F", author="dwimmer")]
    Frequency {
        /// save result to file
        #[clap(display_order=1, short='s', long="save")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=2, short='v', long="valve")]
        valve: bool,
    },
    /*/// Perform harmonic analysis on the input frequency domain to find the 
    /// fundamental frequency and find the harmonics. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(short_flag='H', long_flag="Harmonic", name="H", author="dwimmer")]
    Harmonic {
        /// save result to file
        #[clap(display_order=1, short='s', long="save")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=2, short='v', long="valve")]
        valve: bool,
    },*/
}
