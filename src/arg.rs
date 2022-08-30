use clap::{Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(
    name="RiPPLE", author="dwimmer", version="0.1.6", 
    about="A multi-sensory signal hunting operation.", 
    hide_possible_values=false)]
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
        /// width of one dimensional samples per frame
        #[clap(display_order=1, short='w', long="width", 
            value_parser=clap::value_parser!(u32), default_value="64")]
        width: u32,
        /// generate additional samples along y axis and then collapse, 
        /// via additive synthesis, to single waveform.
        #[clap(display_order=2, short='s', long="square", conflicts_with="cube")]
        square: bool,
        /// generate additional samples along both y and z axis and then collapse, 
        /// via additive synthesis, to single waveform.
        #[clap(display_order=3, short='c', long="cube", conflicts_with="square")]
        cube: bool,
        /// total number of frames to generate.
        #[clap(display_order=4, short='f', long="frames",
            value_parser=clap::value_parser!(u32), default_value="1")]
        frames: u32,
        /// save result to file
        #[clap(display_order=5, short='S', long="save")]
        save: bool,
    },
    /// Generate a frequency domain by performing a Fast Fourier Transform 
    /// on the input time domain and either save to file or pipe to stdout. 
    #[clap(long_flag="Frequency", name="F", author="dwimmer")]
    Frequency {
        /// save result to file
        #[clap(display_order=1, short='S', long="save")]
        save: bool,
    },
    /// Perform harmonic analysis on the input frequency domain to find the 
    /// fundamental frequency and find the harmonics. 
    /// Optionally save to file and/or enable the shutoff valve to stdout.
    #[clap(long_flag="Harmonic", name="H", author="dwimmer")]
    Harmonic {
        /// save result to file
        #[clap(display_order=1, short='S', long="save")]
        save: bool,
    },
    /// Convert .ripl data into .jsonl format then pipe via stdout and/or save to file.
    #[clap(long_flag="Jsonl", name="J", author="dwimmer", disable_help_flag=true)]
    Jsonl {
        /// save result to file
        #[clap(display_order=1, short='S', long="save")]
        save: bool,
    },
    /// Convert .ripl data into .wav format then pipe via stdout and/or save to file.
    #[clap(long_flag="Wav", name="W", author="dwimmer", disable_help_flag=true)]
    Wav {
        /// save result to file
        #[clap(display_order=1, short='S', long="save")]
        save: bool,
    },
}
