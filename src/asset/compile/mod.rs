use std::path::Path;
use std::io::{File, FileMode, FileAccess, FilePermission};
use std::io::fs::{self, PathExtensions};

mod scene;
mod scene_tests;


pub fn compile_path(path: &Path, output_folder: &Path) {
    if path.is_dir() {
        let contents = fs::readdir(path).ok().unwrap();
        for entry in contents.iter() {
            if entry.is_dir() {
                compile_path(entry, output_folder);
            } else {
                compile_asset(entry, output_folder);
            }
        }
    }
    else if path.is_file() {
        compile_asset(path, output_folder);
    }
}

pub fn compile_asset(path: &Path, output_folder: &Path) {
    let ext = path.extension_str()
        .expect("No extension. Cannot determine file type.");

    let mut file = File::open(path)
        .ok().expect("Unable to open file.");

    //Output is relative to output_folder
    let mut output_path = output_folder.clone();
    output_path.push(path);
    output_path.set_extension(get_compiled_extension(ext));
    
    //Create directories + output file
    fs::mkdir_recursive(&output_path.dir_path(), FilePermission::all());
    let mut output_file = File::create(&output_path)
        .ok().expect("Unable to open output file.");

    //Compile the file based on extension
    match ext {
        "scene" => scene::compile_scene(&mut file, &mut output_file),
        _ => { }
    }
}

fn get_compiled_extension(ext: &str) -> &'static str {
    match ext {
        "scene" => "cscene",
        _ => ""
    }
}
