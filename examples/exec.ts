import { blue } from "https://deno.land/std@0.130.0/fmt/colors.ts";
import { exec, execSync } from "../src/ffi.ts";

log("$ pwd");
log("Exit code:", await exec("pwd"));

log("Change cwd to: src");
Deno.chdir("src");

log("$ pwd");
log("Exit code:", await exec("pwd"));

log("$ echo foo bar baz");
log("Exit code:", execSync("echo foo bar baz"));

function log(msg: string, ...args: Array<unknown>) {
  console.log(blue(msg), ...args);
}
