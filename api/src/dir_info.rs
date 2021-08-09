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
pub struct DirContentsInfo {
    path: String,
    size: Option<usize>,
}

#[derive(Serialize)]
pub struct DirContentsFileInfo {
    path: String,
    size: usize,
}

#[derive(Serialize)]
pub struct DirContents {
    pub directories: Vec<DirContentsInfo>,
    pub files: Vec<DirContentsFileInfo>,
}

pub fn ls(path: &Path, show_dir_size: bool) -> io::Result<DirContents> {
    let mut files = vec![];
    let mut dirs = vec![];

    if path.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();

                if sub_path.is_file() {
                    files.push(DirContentsFileInfo {
                        path: str_of_path(&sub_path),
                        size: metadata(sub_path)?.len() as usize,
                    });
                } else {
                    dirs.push(DirContentsInfo {
                        path: str_of_path(&sub_path),
                        size: if show_dir_size {
                            Some(dir_size(&sub_path)?)
                        } else {
                            None
                        },
                    });
                }
            }
        }
    }

    files.sort_by_key(|f| f.size);
    files.reverse();

    if show_dir_size {
        dirs.sort_by_key(|d| d.size);
        dirs.reverse();
    }

    Ok(DirContents {
        files,
        directories: dirs,
    })
}

pub fn dir_size(path: &Path) -> io::Result<usize> {
    let mut total_size = 0;

    if path.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();

                if sub_path.is_file() {
                    let file_size = metadata(&sub_path)?.len() as usize;
                    total_size += file_size;
                } else {
                    total_size += dir_size(&sub_path)?;
                }
            }
        }
    }

    Ok(total_size)
}
