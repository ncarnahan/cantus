extern crate cgmath;
extern crate getopts;
extern crate serialize;
extern crate uuid;

use std::path::Path;
use getopts::{optopt, optflag, getopts};

mod asset;
mod scene;

fn main() {
    let args = std::os::args();
    let opts = [
        optopt("c", "compile", "Compile a asset file or folder", "PATH"),
        optopt("p", "pack", "Pack asset files in a folder", "FOLDER"),
        optopt("o", "output", "Specify output folder", "FOLDER"),
    ];

    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }
    };

    if matches.opt_present("compile") {
        if !matches.opt_present("output") {
            panic!("Output directory not specified.");
        }

        let path = Path::new(matches.opt_str("compile").unwrap());
        let output_folder = Path::new(matches.opt_str("output").unwrap());
        asset::compile::compile_path(&path, &output_folder);
    }
    else if matches.opt_present("pack") {

    }
    else {
        //Run the game
    }
}
