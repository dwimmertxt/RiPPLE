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
    /// Generate a time domain from the output of the RiPPLE function and pipe to stdout. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(long_flag="Time", name="T", author="dwimmer", disable_help_flag=true)]
    Time {
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=1, short='w', long="width", 
            value_parser=clap::value_parser!(u32), default_value="64")]
        width: u32,
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=2, short='h', long="height", 
            value_parser=clap::value_parser!(u32), default_value="64")]
        height: u32,
        /// width of one dimensional samples per frame; square root of total samples
        #[clap(display_order=3, short='d', long="depth", 
            value_parser=clap::value_parser!(u32), default_value="64")]
        depth: u32,
        /// time resolution of ripple; total number of frames
        #[clap(display_order=4, short='r', long="resolution",
            value_parser=clap::value_parser!(u32), default_value="1")]
        resolution: u32,
        /// save result to file
        #[clap(display_order=5, short='s', long="save", conflicts_with="shutsave")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=6, short='S', long="shutsave", conflicts_with="save")]
        shutsave: bool,
    },
    /// Generate a frequency domain by performing a Fast Fourier Transform 
    /// on the input time domain and either save to file or pipe to stdout. 
    #[clap(long_flag="Frequency", name="F", author="dwimmer")]
    Frequency {
        /// save result to file
        #[clap(display_order=1, short='s', long="save", conflicts_with="shutsave")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=2, short='S', long="shutsave", conflicts_with="save")]
        shutsave: bool,
        /// name of file to import
        #[clap(display_order=3, value_parser)]
        file: Vec<String>,
    },
    /// Perform harmonic analysis on the input frequency domain to find the 
    /// fundamental frequency and find the harmonics. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(short_flag='H', long_flag="Harmonic", name="H", author="dwimmer")]
    Harmonic {
        /// save result to file
        #[clap(display_order=1, short='s', long="save", conflicts_with="shutsave")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=2, short='S', long="shutsave", conflicts_with="save")]
        shutsave: bool,
        /// name of file to import
        #[clap(display_order=3, value_parser)]
        file: Vec<String>,
    },
    /// Convert data provided via stdin or file to jsonl and then pipe to stdout. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(long_flag="Jsonl", name="J", author="dwimmer", disable_help_flag=true)]
    Jsonl {
        /// save result to file
        #[clap(display_order=8, short='s', long="save", conflicts_with="shutsave")]
        save: bool,
        /// pipe shutoff valve; nothing sent to stdout.
        #[clap(display_order=9, short='S', long="shutsave", conflicts_with="save")]
        shutsave: bool,
        /// name of file to import
        #[clap(display_order=10, value_parser)]
        file: Vec<String>,
    },
}
