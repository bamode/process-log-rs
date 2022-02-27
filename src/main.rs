/// Brent Mode
/// <bmode@wisc.edu>
/// University of Wisconsin - Madison
/// Department of Physics
/// Wisconsin IceCube Particle Astrophysics Center
/// 
/// Created 25 February 2022
/// Current version: 0.2.0
/// Status: operative

use std::fs::File;
use std::io::{Read, Write};

use clap::Parser;

fn main() {
    // handle args
    let cli = Cli::parse();
    let filename = cli.file;
    let mut file = File::open(&filename).unwrap();
    let mut contents = String::new(); 
    file.read_to_string(&mut contents).unwrap();
    let (_, res) = contents.split_once("\n").unwrap(); // remove the header
    contents = String::from(res);
    let file_header = filename
        .split("/") // split on pathing (in *NIX systems)
        .collect::<Vec<&str>>() // collect split into a `Vec` of string slices
        .pop().unwrap() // get just the last element, e.g. the filename
        .split(".") // split the filename from extensions
        .collect::<Vec<&str>>()[0]; // just keep the important part of the name

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
    let mut new_file = File::create(format!("{}-cal-list.txt", file_header)).unwrap();
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
            let mut new_file = File::create(format!("{}-ramp-{}-tf-dac-list.txt", file_header, ramp)).unwrap();
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

/// Command line interface definition so that help is provided and I don't
/// have to try to parse both a flag and a required positional argument on my own.
#[derive(Parser)]
#[clap(name = "process-log-rs",
       author = "Brent Mode <bmode@wisc.edu>",
       version,
       about = "Process log file for ramp current studies.",
       long_about = None)]
struct Cli {
    file: String,
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
