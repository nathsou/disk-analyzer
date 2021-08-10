use super::biggest::{Biggest, DocInfo};
use serde::{Deserialize, Serialize};
use std::fs::{read_dir, symlink_metadata};
use std::io;
use std::path::Path;
use std::sync::Arc;

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
                let md = symlink_metadata(&sub_path)?;

                if md.is_file() {
                    let file_size = md.len() as usize;
                    total_size += file_size;
                    files_count += 1;

                    if biggest_files.is_big_enough(file_size) {
                        biggest_files.insert(DocInfo {
                            path: format!("{}", str_of_path(&sub_path)),
                            size: file_size,
                        });
                    }
                } else if md.is_dir() {
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
    pub size: usize,
}

pub fn ls(path: &Path, show_dir_size: bool, db: Arc<sled::Db>) -> io::Result<DirContents> {
    let mut files = vec![];
    let mut dirs = vec![];

    if path.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();
                let md = symlink_metadata(&sub_path)?;

                if md.is_file() {
                    files.push(DirContentsFileInfo {
                        path: str_of_path(&sub_path),
                        size: md.len() as usize,
                    });
                } else if md.is_dir() {
                    dirs.push(DirContentsInfo {
                        path: str_of_path(&sub_path),
                        size: if show_dir_size {
                            Some(dir_size(&sub_path, db.clone())?)
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

    let files_size: usize = files.iter().map(|f| f.size).sum();
    let dirs_size: usize = dirs.iter().map(|d| d.size.unwrap_or(0)).sum();

    Ok(DirContents {
        files,
        directories: dirs,
        size: files_size + dirs_size,
    })
}

#[derive(Serialize, Deserialize)]
struct DirectorySize {
    size: usize,
    last_modified: std::time::SystemTime,
}

pub fn dir_size(path: &Path, db: Arc<sled::Db>) -> io::Result<usize> {
    let key = str_of_path(path);
    let md = symlink_metadata(path)?;
    let last_modified = md.modified()?;

    if let Some(data) = db.get(key.clone())? {
        let ds: DirectorySize =
            bincode::deserialize(&data).expect("Could not deserialize from database");

        if ds.last_modified >= last_modified {
            return Ok(ds.size);
        }
    }

    let mut total_size = 0;

    if md.is_dir() {
        if let Ok(entries) = read_dir(path) {
            for entry in entries {
                let sub_path = entry?.path();

                match symlink_metadata(&sub_path) {
                    Ok(md) => {
                        if md.is_file() {
                            let file_size = md.len() as usize;
                            total_size += file_size;
                        } else if md.is_dir() {
                            total_size += dir_size(&sub_path, db.clone())?;
                        }
                    }
                    Err(err) => {
                        println!("Error on path {} : {}", path.display(), err);
                    }
                }
            }
        }
    }

    db.insert(
        key,
        bincode::serialize(&DirectorySize {
            size: total_size,
            last_modified,
        })
        .expect("Could not serialize directory info"),
    )?;

    Ok(total_size)
}
