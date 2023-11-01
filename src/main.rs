use std::path::PathBuf;
use structopt::StructOpt;
use trash;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "trash_rm")]
struct Opt {

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    for file in opt.files {
        let path = file.as_path().as_os_str().to_string_lossy();
        if !file.exists() {
            // print error
            eprintln!("trash_rm: {path}: No such file or directory");
            continue;
        }
        match trash::delete(&file) {
            Ok(_) => {}
            Err(e) => {
                // print error
                eprintln!("trash_rm: {path}: {e}");
            }
        }
    }
}
