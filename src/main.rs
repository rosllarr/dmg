use std::path::Path;
use std::fs::File;
use std::io::{Result, Write, Read};

#[derive(Debug)]
struct FileOverwrite<'a> {
    src: &'a Path,
    dest: &'a Path,
}

impl<'a> FileOverwrite<'a> {
    fn copy(&mut self) -> Result<()> {
        let mut src = File::open(self.src)?;
        let mut dest = File::create(self.dest)?;
        let mut buffer = Vec::new();
        src.read_to_end(&mut buffer)?;
        dest.write_all(&buffer)?;
        Ok(())
    }
}

fn compare_modify_time<'a>(path1: &'a Path, path2: &'a Path) -> Result<FileOverwrite<'a>> {
    let mod_time1 = path1.metadata()?.modified()?;
    let mod_time2 = path2.metadata()?.modified()?;

    if mod_time1 > mod_time2 {
        Ok(
            FileOverwrite {
                src: path1,
                dest: path2,
            }
        )
    } else if mod_time2 > mod_time1 {
        Ok(
            FileOverwrite {
                src: path2,
                dest: path1,
            }
        )
    } else {
        panic!()
    }
}


fn main() {
    let path1 = Path::new("test1");
    let path2 = Path::new("test2");
    let mut result = compare_modify_time(path1, path2).unwrap();
    let _ = result.copy();
}
