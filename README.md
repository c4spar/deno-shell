# deno-shell

Execute deno shell commands in deno.

```ts
import { exec, execSync } from "./mod.ts";

await exec("pwd");

execSync("pwd");
```
