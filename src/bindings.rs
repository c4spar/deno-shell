use anyhow::Context;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn deno_shell_exec_sync(script: *const c_char) -> i32 {
    let script_str = unsafe {
        CStr::from_ptr(script)
            .to_str()
            .expect("No null bytes in parameter script")
    };
    execute_task_sync(script_str)
}

fn execute_task_sync(script: &str) -> i32 {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(execute_task(script))
        .unwrap()
}

async fn execute_task(script: &str) -> Result<i32, anyhow::Error> {
    let seq_list = deno_task_shell::parser::parse(script)
        .with_context(|| format!("Error parsing script '{}'.", script))?;

    let cwd = std::env::current_dir()?;
    let env_vars = std::env::vars().collect::<std::collections::HashMap<String, String>>();

    let exit_code = deno_task_shell::execute(seq_list, env_vars, &cwd).await;

    Ok(exit_code)
}
