/// Brent Mode
/// <bmode@wisc.edu>
/// University of Wisconsin - Madison
/// Department of Physics
/// Wisconsin IceCube Particle Astrophysics Center
/// 
/// Created 25 February 2022
/// Current version: 0.1.0
/// Status: operative

use std::env::args;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // handle args; should replace with some clap stuff as this doesn't even support -h
    let mut args = args();
    if args.len() < 2 { usage(); std::process::exit(1); }
    if args.len() > 2 { usage(); println!("too many arguments provided"); std::process::exit(1); };
    args.next().unwrap();
    let filename = args.next().unwrap();

    let mut file = File::open(filename).unwrap(); // hard-coded file name for now
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();
    let (_, res) = contents.split_once("\n").unwrap(); // remove the header
    contents = String::from(res);

    let (mut ramps, runs, vpeds) = parse_log(&contents); // ramps is mutable because it has duplicates in the logs
    ramps.dedup(); // remove duplicates

    let mut bools = Vec::new();
    for i in 0..vpeds.len() - 1 {
        bools.push(vpeds[i + 1] - vpeds[i] > 0); // find the transitions from one isel to another by looking at vpeds
    }
    bools.push(true); // make the vector lengths equal

    let mut output: String = String::new(); // make a string to buffer the output
    for i in 0..runs.len() {
        let run = &runs[i]; // needs to be borrowed because a `String` is not `Copy`
        // `push_str` is roughly equivalent in this scenario to `push` on a `Vec`
        output.push_str(&format!("/data/wipac/CTA/target5and7data/runs_320000_through_329999/cal{}.r1\n", run));
    }
    
    // make the output file and unwrap, handling of permissions errors should be supported
    let mut new_file = File::create("2021-12-22-ramplog-cal-list.txt").unwrap();
    new_file.write_all(output.trim_end().as_bytes()).unwrap(); // have to write output as bytes
    
    output = String::new(); // reset our output buffer
    let mut iramp = ramps.iter(); // we want to write out files whenever we change data sets, so we iterate ramps separately
    for i in 0..runs.len() {
        let run = &runs[i];
        let vped = vpeds[i];
        let b = bools[i];
        output.push_str(&format!("/data/wipac/CTA/target5and7data/runs_320000_through_329999/cal{}.r1 {}\n", run, vped));
        if !b {
            let ramp = iramp.next().unwrap();
            let mut new_file = File::create(format!("2021-12-22-ramplog-ramp-{}-tf-dac-list.txt", ramp)).unwrap();
            new_file.write_all(output.trim_end().as_bytes()).unwrap();
            output = String::new();
        }
    }
}

/// A simple function for parsing the log files that are currently produced in the
/// CTA - SCT laboratory at the University of Wisconsin in my study of component electronics
/// effect on SNR. It takes a reference to the contents of a log file and returns the component
/// values in the form of more directly usable types. 
fn parse_log(contents: &String) -> (Vec<u64>, Vec<String>, Vec<isize>) {
    let mut ramps: Vec<u64> = Vec::new();
    let mut runs: Vec<String> = Vec::new();
    let mut vpeds: Vec<isize> = Vec::new();
    for line in contents.lines() {
        let l: Vec<&str> = line.split(",").collect();
        ramps.push(l[1].parse().unwrap());
        runs.push(String::from(l[0]));
        vpeds.push(l[2].parse().unwrap());
    }

    (ramps, runs, vpeds)
}

fn usage() {
    let name = "process-log-rs";
    let author = "Brent Mode";
    let email = "<bmode@wisc.edu>";
    let usage = "Usage: process-log FILE\nProcess log file for ramp current studies.";
    let output = format!("{}\n{}\n{}\n{}\n", name, author, email, usage);
    println!("{}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_log() {
        let contents = String::from("300000,0,123\n300001,1,124");
        let (ramps, runs, vpeds) = parse_log(&contents);
        assert_eq!(ramps.len(), 2);
        assert_eq!(runs.len(), 2);
        assert_eq!(vpeds.len(), 2);
        assert_eq!(runs[0], "300000");
        assert_eq!(ramps[0], 0u64);
        assert_eq!(vpeds[0], 123isize);
    }
}
