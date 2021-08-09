use super::biggest::{Biggest, DocInfo};
use super::dir_info::{dir_info, ls};
use serde::{Deserialize, Serialize};
use std::path::Path;
use warp::{Filter, Rejection};

#[derive(Deserialize)]
pub struct DirInfoParams {
    pub path: Option<String>,
    pub files_count: Option<usize>,
    pub dirs_count: Option<usize>,
}

#[derive(Serialize)]
struct ErrorMessage {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
struct DirInfoRes {
    pub path: String,
    pub size: usize,
    pub files_count: usize,
    pub duration: u128,
    pub biggest_dirs: Vec<DocInfo>,
    pub biggest_files: Vec<DocInfo>,
}

fn error_msg(msg: String) -> Result<warp::reply::Json, warp::Rejection> {
    Ok(warp::reply::json(&ErrorMessage { message: msg }))
}

async fn get_dir_info(params: DirInfoParams) -> Result<impl warp::Reply, Rejection> {
    let files_count = params.files_count.unwrap_or(10);
    let dirs_count = params.dirs_count.unwrap_or(10);
    let mut biggest_dirs = Biggest::new(dirs_count);
    let mut biggest_files = Biggest::new(files_count);

    if let Some(path) = params.path {
        let start = std::time::Instant::now();
        match dir_info(Path::new(&path), &mut biggest_dirs, &mut biggest_files) {
            Ok(info) => {
                println!(
                    "dir sorts: {}, file sorts: {}",
                    biggest_dirs.sort_count(),
                    biggest_files.sort_count()
                );
                Ok(warp::reply::json(&DirInfoRes {
                    path,
                    size: info.size,
                    files_count: info.files_count,
                    duration: start.elapsed().as_millis(),
                    biggest_dirs: biggest_dirs.values(),
                    biggest_files: biggest_files.values(),
                }))
            }
            Err(err) => error_msg(format!("{}", err)),
        }
    } else {
        error_msg(String::from("'path' is required"))
    }
}

#[derive(Deserialize)]
pub struct DirContentsParams {
    pub path: Option<String>,
    pub show_dir_size: bool,
}

async fn get_dir_contents(params: DirContentsParams) -> Result<impl warp::Reply, Rejection> {
    if let Some(path) = params.path {
        match ls(Path::new(&path), params.show_dir_size) {
            Ok(contents) => Ok(warp::reply::json(&contents)),
            Err(err) => error_msg(format!("{}", err)),
        }
    } else {
        error_msg(String::from("'path' is required"))
    }
}

pub async fn serve() {
    // GET /dir?path=/Users/nathan
    let dir_req = warp::path!("dir")
        .and(warp::get())
        .and(warp::query::<DirInfoParams>())
        .and_then(get_dir_info);

    // GET /ls?path=/Users/nathan
    let ls_req = warp::path!("ls")
        .and(warp::get())
        .and(warp::query::<DirContentsParams>())
        .and_then(get_dir_contents);

    let routes = dir_req.or(ls_req).with(warp::log("dev"));

    println!("Listening on port 3030");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
