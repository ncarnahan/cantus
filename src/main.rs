extern crate cgmath;
extern crate getopts;
extern crate serialize;
extern crate uuid;

use std::old_path::Path;
use getopts::Options;

mod asset;
mod scene;

fn main() {
    let args = std::os::args();
    let mut opts = Options::new();
    opts.optopt("c", "compile", "Compile a asset file or folder", "PATH");
    opts.optopt("p", "pack", "Pack asset files in a folder", "FOLDER");
    opts.optopt("o", "output", "Specify output folder", "FOLDER");

    let matches = match opts.parse(args.tail()) {
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
