use crate::arg;

use crate::export;
use crate::frequency;
use crate::harmonic;
use crate::time;



pub fn generate(args: &arg::Prism) {
    
    let mut time_domains      = Vec::new();
    let mut frequency_domains = Vec::new();
    let mut harmonic_domain   = Vec::new();

    for t in 0..args.resolution {time_domains.push(time::domain(&args.norm, &args.width, &args.height, t));}
    if args.fft {
        for td in &time_domains {frequency_domains.push(frequency::domain(&td));}
    }
    if args.harmonics {
        for fd in &frequency_domains {harmonic_domain.push(harmonic::fundamental(&fd));}
    }
    if args.jsonl || !args.pipe.is_empty() {
        export::prism(
            &args, &vec![harmonic_domain], &vec![time_domains, frequency_domains]);
    }
}
