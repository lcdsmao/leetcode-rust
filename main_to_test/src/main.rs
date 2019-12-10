use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

fn main() -> io::Result<()> {
    let problem_dir = Path::new("./problem");
    for entry in fs::read_dir(problem_dir)? {
        let src_path = entry?.path().join("src");
        let lib_file = src_path.join("lib.rs");
        let main_file = src_path.join("main.rs");
        if main_file.is_file() && lib_file.is_file() {
            move_main_body_to_lib_test(main_file.as_path(), lib_file.as_path())?;
            fs::remove_file(main_file)?;
        }
    }
    Ok(())
}

fn move_main_body_to_lib_test(main_file_path: &Path, lib_file_path: &Path) -> io::Result<()> {
    lazy_static! {
        static ref MAIN_BODY_RE: Regex = Regex::new(r"^(?s).*main.*?\{.*?\n(.*)\n}\n?$").unwrap();
    }
    let content = fs::read_to_string(main_file_path)?;
    if let Some(cap) = MAIN_BODY_RE.captures(&content) {
        let cap: &str = &cap[1];
        let mut lib_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(lib_file_path)?;
        lib_file.write("\n".as_bytes())?;
        lib_file.write("#[cfg(test)]\n".as_bytes())?;
        lib_file.write("mod tests {\n".as_bytes())?;
        lib_file.write("    use super::*;\n".as_bytes())?;
        lib_file.write("    #[test]\n".as_bytes())?;
        lib_file.write("    fn it_works() {\n".as_bytes())?;
        lib_file.write(cap.as_bytes())?;
        lib_file.write("    }\n}\n".as_bytes())?;
    } else {
        println!("Parse error for file {:?}", main_file_path);
    }
    Ok(())
}
