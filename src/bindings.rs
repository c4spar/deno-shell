use serde::Deserialize;
use serde::Serialize;
use std::ffi::CString;
use std::os::raw::c_char;
use std::path::Path;
use std::string::String;

#[repr(C)]
#[derive(Default, Deserialize)]
pub struct ExecOptions {
    script: String,
    cwd: Option<String>,
}

#[repr(C)]
#[derive(Default, Serialize)]
pub struct ExecResult {
    exit_code: i32,
    error_message: Option<String>,
}

#[no_mangle]
pub extern "C" fn deno_shell_exec_sync(ptr: *const u8, len: usize) -> *const c_char {
    let options = decode::<ExecOptions>(ptr, len);
    let str = execute_task_sync(&options).unwrap();

    CString::new(str).unwrap().into_raw()
}

fn execute_task_sync(options: &ExecOptions) -> Result<String, anyhow::Error> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(execute_task(options))
}

async fn execute_task(options: &ExecOptions) -> Result<String, anyhow::Error> {
    let seq_list = deno_task_shell::parser::parse(&options.script);

    if seq_list.is_err() {
        let options = ExecResult {
            exit_code: 1,
            error_message: Option::Some(format!(
                "{} '{}'. {}",
                "Failed to parse script",
                options.script,
                seq_list.err().unwrap().to_string()
            )),
        };
        return Ok(serde_json::to_string(&options)?);
    }

    let seq_list = seq_list.unwrap();

    // if options.cwd.is_some() {
    //     if ! Path::new(&options.cwd.unwrap()).exists() {
    //         let options = ExecResult {
    //             exit_code: 1,
    //             error_message: Option::Some(format!(
    //                 "{} '{}'.",
    //                 "No such file or directory",
    //                 options.cwd.unwrap(),
    //             )),
    //         };
    //         return Ok(serde_json::to_string(&options)?)
    //     }
    // }

    let cwd = match &options.cwd {
        Some(x) => {
            let path = Path::new(x);
            if Path::is_relative(path) {
                let path = Path::join(std::env::current_dir().unwrap().as_path(), path);
                path.canonicalize().unwrap().to_path_buf()
            } else {
                path.canonicalize().unwrap().to_path_buf()
            }
        }
        None => std::env::current_dir()?,
    };

    let env_vars = std::env::vars().collect::<std::collections::HashMap<String, String>>();

    let exit_code = deno_task_shell::execute(seq_list, env_vars, &cwd).await;

    let options = ExecResult {
        exit_code,
        error_message: None,
    };

    Ok(serde_json::to_string(&options)?)
}

fn decode<'a, T: Deserialize<'a>>(ptr: *const u8, len: usize) -> T {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let buf_str = std::str::from_utf8(buf).unwrap();
    serde_json::from_str::<'a, T>(buf_str).unwrap() as T
}
