extern crate inotify;

use inotify::{Inotify, WatchMask};
use std::path::Path;
use std::path::PathBuf;
use std::env;
use std::io::Write;

fn get_dirname() -> Result<(PathBuf, PathBuf), (String, i32)> {
    let mut args = env::args();
    let exe_name = match args.next() {
        None => panic!("Expected argv of size larger than or equal to 1."),
        Some(n) => n
    };
    let maybe_filename: Result<String, (String, i32)> = match args.next() {
        None => return Err((format!("Usage: {} <filename>", exe_name), 1)),
        Some(n) => Ok(n)
    };
    let maybe_dirname = maybe_filename.map(|filename| {
        let path = Path::new(&filename);
        match path.parent() {
            Some(d) if d.is_absolute() => Ok((d.to_path_buf(), path.to_path_buf())),
            Some(relative_path) => {
                match env::current_dir() {
                    Ok(cwd) => Ok((cwd.join(relative_path), cwd.join(path))),
                    Err(_) => Err((String::from("Current working directory is invalid."), 3)),
                }
            },
            None => {
                Err((format!("Usage: {} <filename>", exe_name), 1))
            }
        }
    });
    match maybe_dirname {
        Ok(dirs) => dirs,
        Err(x) => Err(x)
    }
}

fn wait_for(dirname: &Path, full_dir_name: &Path) -> () {
    let mut ino = Inotify::init().unwrap();
    ino.add_watch(&dirname, WatchMask::CREATE).unwrap();
    if !full_dir_name.exists() {
        loop {
            let mut buffer = [0; 1024];
            let events = ino.read_events_blocking(&mut buffer)
                .expect("Error while reading events");
            for event in events {
                match event.name {
                    Some(name) => {
                        if dirname.join(name) == full_dir_name {
                            return;
                        }
                    },
                    // TODO if the watched directory is deleted, we should quit
                    // with a non-zero exit code.
                    None => ()
                }
            }
        }
    }
}

fn main() {

    let (dirname, full_dir_name) = match get_dirname() {
        Ok(d) => d,
        Err((reason, code)) => {
            let _ = writeln!(&mut std::io::stderr(), "{}", &reason);
            std::process::exit(code);
        }
    };
    wait_for(&dirname, &full_dir_name);
}
