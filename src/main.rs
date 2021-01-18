#[macro_use] extern crate log;
extern crate stderrlog;

use lazy_static::lazy_static;
use regex::Regex;
use std::path::PathBuf;
use std::u16;
use structopt::StructOpt;

/// Parse target ratio from user input or die.
fn parse_ratio(ratio: &str) -> (u16, u16) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+):(\d+)").unwrap();
        static ref ERR: String = "Cannot parse ratio. Ensure ratio is in format of w:h, e.g.: '16:9'".to_string();
    }
    let matches = RE.captures(ratio).expect(&ERR);
    let w: u16 = matches.get(1)
                        .map(|w| w.as_str())
                        .expect(&ERR)
                        .parse()
                        .expect(&ERR);
    let h: u16 = matches.get(2)
                        .map(|m| m.as_str())
                        .expect(&ERR)
                        .parse()
                        .expect(&ERR);
    return (w, h)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Paddington", about = "Add padding or crop images to fit an aspect ratio.")]
struct Opt {
    #[structopt(parse(from_str = parse_ratio),
                help = "Ratio of the output image. Formatted as 'width:height', e.g. '4:3'.")]
    ratio: (u16, u16),
    #[structopt(parse(from_os_str),
                help = "Path to input image.")]
    input: PathBuf,
    #[structopt(parse(from_os_str),
                help = "Path to output image.")]
    output: PathBuf,
    #[structopt(short, long,
                help = "Crop the image down to the target ratio instead of adding padding.")]
    crop: bool,
    #[structopt(short, long,
                parse(from_occurrences),
                help = "Verbosity of output. e.g. -v, -vv, -vvv, etc.")]
    verbose: usize,
    #[structopt(short, long,
                help = "Do not print anything to stdout. Supersedes the --verbose flag.")]
    quiet: bool,
}

/// Get CLI options, initilize logging.
fn init() -> Opt {
    let opt = Opt::from_args();
    stderrlog::new()
        .module(module_path!())
        .quiet(opt.quiet)
        .verbosity(opt.verbose)
        .init()
        .unwrap();
    trace!("Options initialized: {:?}", opt);
    return opt
}

fn main() {
    let opts: Opt = init();
    println!("Flags: {:?}", opts);
}


// ========== BIN UNIT TESTS ==========

#[test]
fn parse_ratio_test() {
    let (w, h) = parse_ratio("16:9");
    assert!(w == 16);
    assert!(h == 9);
}

#[test]
#[should_panic]
fn parse_ratio_panic_test() {
    let (_w, _h) = parse_ratio("💩");
    unreachable!()
}
