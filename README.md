# deno-shell

Deno bindings for [deno_task_shell](https://crates.io/crates/deno_task_shell).

Execute deno shell commands in deno.

```ts
import { exec, execSync } from "./mod.ts";

// Execute task asynchronous.
await exec("pwd");

// Execute task synchronous.
execSync("pwd");

// Execute task with specified cwd.
await exec("pwd", { cwd: "examples" });
```
