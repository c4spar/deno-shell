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
    exit_code: Option<i32>,
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
        let message = format!(
            "{} '{}'. {}",
            "Failed to parse script",
            options.script,
            seq_list.err().unwrap().to_string()
        );
        return Ok(error_result(message)?);
    }

    let seq_list = seq_list.unwrap();

    if options.cwd.is_some() {
        let cwd = options.cwd.as_ref().unwrap();
        if !Path::new(&cwd).exists() {
            let message = format!("{} '{}'.", "No such file or directory", cwd);
            return Ok(error_result(message)?);
        }
    }

    let cwd = match &options.cwd {
        Some(x) => {
            let path = Path::new(x);
            if Path::is_relative(path) {
                Path::join(std::env::current_dir().unwrap().as_path(), path)
            } else {
                path.to_path_buf()
            }
            .canonicalize()
            .unwrap()
        }
        None => std::env::current_dir()?,
    };

    let env_vars = std::env::vars().collect::<std::collections::HashMap<String, String>>();

    let exit_code = deno_task_shell::execute(seq_list, env_vars, &cwd).await;

    Ok(success_result(exit_code)?)
}

fn success_result(exit_code: i32) -> serde_json::error::Result<String> {
    result(None, Option::Some(exit_code))
}

fn error_result(message: String) -> serde_json::error::Result<String> {
    result(Option::Some(message), None)
}

fn result(
    error_message: Option<String>,
    exit_code: Option<i32>,
) -> serde_json::error::Result<String> {
    let options = ExecResult {
        exit_code,
        error_message,
    };
    serde_json::to_string(&options)
}

fn decode<'a, T: Deserialize<'a>>(ptr: *const u8, len: usize) -> T {
    let buf = unsafe { std::slice::from_raw_parts(ptr, len) };
    let buf_str = std::str::from_utf8(buf).unwrap();
    serde_json::from_str::<'a, T>(buf_str).unwrap() as T
}
