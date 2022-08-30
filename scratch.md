ripl Dt | ripl -Ib -Ob Df | ripl -Ib -Oj Dh

ripl D T -w16 -r8 -j -Pb | ripl D F -j -Pb | ripl D H -j -Pj 


ripl conv "freq.jsonl" > "freq.ripl"
ripl C -tspr "time.jsonl" | ripl F -p | ripl H -sp | ripl C -hpj | vd
ripl R -w32 -r16 -p | ripl F -s 

ripl T -w32 -r16 P | ripl x 

if stdin {
    header = stdin.read(0..5);
    if header == "RIPL" {
        all g
    }

}

.ripl format specification:


time domain: 
- width
- resolution


LE | 04 | format ID; ASCII chars  "R", "i", "P", "L"
LE | 04 | domain ID; ASCII chars ["T", "I", "M", "E"], ["F", "R", "E", "Q"], ... 
LE | 04 | start data; ASCII chars "DATA"


Domains:

--TIME--
LE | 02 | width; u16
LE | 02 | height; u16
LE | 02 | depth; u16
LE | 02 | resolution; u16
LE | 01 | bits_per_amp; u8
//LE | 08 | bytes_total; u64
//LE | 05 | amps_total; u64



--FREQ--
LE | 05 | mags_total; u64
LE | 01 | bits_per_mag; u8
LE | 08 | bytes_total; u64

--HARM--
LE | 00 | 


--DATA--

---------------------------

## make ripple into more of an intelligent general function
q) how? what does that mean?
a) retain ability to specify subcommand via clap; take advantage of ripl 
   encoding to determine input data format. 

if .ripl:
- process "next step" e.g. if input == time: return frequency domain