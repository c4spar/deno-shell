import { blue } from "https://deno.land/std@0.130.0/fmt/colors.ts";
import { exec, execSync } from "../src/ffi.ts";

log("$ pwd");
log("Exit code:", await exec("pwd"));

log("$ pwd");
log("Exit code:", execSync("pwd", { cwd: "examples" }));

log("Change dir: src");
Deno.chdir("src");
log("$ pwd");
log("Exit code:", await exec("pwd"));

function log(msg: string, ...args: Array<unknown>) {
  console.log(blue(msg), ...args);
}
