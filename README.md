# deno-shell

Deno bindings for [deno_task_shell](https://crates.io/crates/deno_task_shell).

Execute deno shell commands in deno.

```ts
import { exec, execSync } from "./mod.ts";

await exec("pwd");

execSync("pwd");
```
