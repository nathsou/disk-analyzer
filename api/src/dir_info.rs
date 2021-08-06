use super::biggest::{Biggest, DocInfo};
use serde::Serialize;
use std::fs::{metadata, read_dir};
use std::io;
use std::path::Path;

pub struct DirInfo {
    pub size: usize,
    pub files_count: usize,
}

fn str_of_path(path: &Path) -> String {
    format!("{}", path.display())
}

pub fn dir_info(
    path: &Path,
    biggest_dirs: &mut Biggest,
    biggest_files: &mut Biggest,
) -> io::Result<DirInfo> {
    let mut total_size = 0;
    let mut files_count = 0;

    if path.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();

                if sub_path.is_file() {
                    let file_size = metadata(&sub_path)?.len() as usize;
                    total_size += file_size;
                    files_count += 1;

                    if biggest_files.is_big_enough(file_size) {
                        biggest_files.insert(DocInfo {
                            path: format!("{}", str_of_path(&sub_path)),
                            size: file_size,
                        });
                    }
                } else {
                    let info = dir_info(&sub_path, biggest_dirs, biggest_files)?;
                    total_size += info.size;
                    files_count += info.files_count;

                    if biggest_dirs.is_big_enough(info.size) {
                        biggest_dirs.insert(DocInfo {
                            path: format!("{}", str_of_path(&sub_path)),
                            size: info.size,
                        });
                    }
                }
            }
        }
    }

    Ok(DirInfo {
        size: total_size,
        files_count,
    })
}

#[derive(Serialize)]
pub struct DirContents {
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

pub fn ls(path: &Path) -> io::Result<DirContents> {
    let mut files = vec![];
    let mut dirs = vec![];

    if path.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();

                if sub_path.is_file() {
                    files.push(str_of_path(&sub_path));
                } else {
                    dirs.push(str_of_path(&sub_path));
                }
            }
        }
    }

    Ok(DirContents {
        files,
        directories: dirs,
    })
}
