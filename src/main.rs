extern crate getopts;
extern crate serialize;
extern crate uuid;

use std::path::Path;
use getopts::{optopt, optflag, getopts};

mod asset;

fn main() {
    let args = std::os::args();
    let opts = [
        optopt("c", "compile", "Compile a single asset file", "FILE"),
        optopt("p", "pack", "Pack asset files in a folder", "FOLDER")
    ];

    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("compile") {
        let path = Path::new(matches.opt_str("compile").unwrap());
        asset::compile::compile_asset(path);
    }
    else if matches.opt_present("pack") {

    }
    else {
        //Run the game
    }
}
