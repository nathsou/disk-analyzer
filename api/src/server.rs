use super::biggest::{Biggest, DocInfo};
use super::dir_info::{dir_info, ls};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use warp::{Filter, Rejection};
use webbrowser;

#[derive(Deserialize)]
pub struct DirInfoParams {
    pub path: Option<String>,
    pub files_count: Option<usize>,
    pub dirs_count: Option<usize>,
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

#[derive(Serialize, Debug)]
struct ErrorMessage {
    pub message: String,
}

impl warp::reject::Reject for ErrorMessage {}

fn error_msg(msg: &str) -> Result<warp::reply::Json, warp::Rejection> {
    Err(warp::reject::custom(ErrorMessage {
        message: msg.to_owned(),
    }))
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
            Err(err) => error_msg(&format!("{}", err)),
        }
    } else {
        error_msg("'path' is required")
    }
}

#[derive(Deserialize)]
pub struct DirContentsParams {
    pub path: Option<String>,
    pub show_dir_size: bool,
}

async fn get_dir_contents(
    params: DirContentsParams,
    db: Arc<sled::Db>,
) -> Result<impl warp::Reply, Rejection> {
    if let Some(path) = params.path {
        match ls(Path::new(&path), params.show_dir_size, db) {
            Ok(contents) => Ok(warp::reply::json(&contents)),
            Err(err) => error_msg(&format!("{}", err)),
        }
    } else {
        error_msg("'path' is required")
    }
}

#[derive(Serialize)]
struct OSInfo {
    home: String,
    root: String,
    os: String,
}

async fn get_os_info() -> Result<impl warp::Reply, Rejection> {
    match dirs::home_dir() {
        Some(home) => Ok(warp::reply::json(&OSInfo {
            home: format!("{}", home.display()),
            os: format!("{}", std::env::consts::OS),
            root: String::from(if std::env::consts::OS == "windows" {
                r"C:\"
            } else {
                "/"
            }),
        })),
        None => error_msg("Could not retrieve your home directory"),
    }
}

pub async fn serve() {
    let db = Arc::new(sled::open("./directory_sizes").expect("Could not open local database"));

    // GET /dir?path=/Users/nathan
    let dir_req = warp::path!("dir")
        .and(warp::get())
        .and(warp::query::<DirInfoParams>())
        .and_then(get_dir_info);

    // GET /ls?path=/Users/nathan
    let ls_req = warp::path!("ls")
        .and(warp::get())
        .and(warp::query::<DirContentsParams>())
        .and_then(move |params| get_dir_contents(params, db.clone()));

    let os_info_req = warp::path!("os_info")
        .and(warp::get())
        .and_then(get_os_info);

    let api = warp::path("api")
        .and(dir_req.or(ls_req).or(os_info_req))
        .with(warp::log("dev"));

    let port = 7621;

    println!("disk-analyzer is running on http://localhost:{}/", port);

    let executable_path = std::env::current_exe().expect("Could not get the executable path");
    let executable_dir = executable_path
        .parent()
        .expect("Could not get the executable's parent directory");

    std::env::set_current_dir(executable_dir)
        .expect("Could not change the current working directory");

    let spa_index = "front/index.html";
    let front = warp::fs::dir("front").or(warp::any().and(warp::fs::file(spa_index)));
    let routes = api.or(front);

    match webbrowser::open(&format!("http://localhost:{}", port)) {
        Err(err) => println!("Could not open the web browser: {}", err),
        _ => (),
    }

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
