import { red } from "../deps.ts";

enum LibSuffix {
  windows = "dll",
  darwin = "dylib",
  linux = "so",
}

const encoder = new TextEncoder();
const libSuffix = LibSuffix[Deno.build.os];
const libName = `./target/debug/libdeno_shell.${libSuffix}`;

const dylib = Deno.dlopen(libName, {
  "deno_shell_exec": {
    name: "deno_shell_exec_sync",
    parameters: ["pointer", "usize"],
    result: "pointer",
    nonblocking: true,
  },
  "deno_shell_exec_sync": {
    parameters: ["pointer", "usize"],
    result: "pointer",
  },
});

interface ExecOptions {
  cwd?: string;
}

interface ExecResult {
  exit_code?: number;
  error_message?: string;
}

export async function exec(
  script: string,
  options: ExecOptions = {},
): Promise<number> {
  const pointer = await dylib.symbols.deno_shell_exec(
    ...encodeJson({
      cwd: Deno.cwd(),
      ...options,
      script,
    }),
  );
  return parseExecResult(pointer);
}

export function execSync(script: string, options: ExecOptions = {}): number {
  const pointer = dylib.symbols.deno_shell_exec_sync(
    ...encodeJson({
      cwd: Deno.cwd(),
      ...options,
      script,
    }),
  );
  return parseExecResult(pointer);
}

function encodeJson(val: unknown): [Uint8Array, number] {
  const objectStr = JSON.stringify(val);
  const buf = encoder.encode(objectStr);
  return [buf, buf.length];
}

function parseExecResult(pointer: Deno.UnsafePointer): number {
  const view = new Deno.UnsafePointerView(pointer);
  const cStr = view.getCString();
  const result: ExecResult = JSON.parse(cStr);
  if (result.error_message) {
    throw new Error(red(result.error_message));
  }
  if (typeof result.exit_code === "undefined") {
    throw new Error(red("Internal error"));
  }
  return result.exit_code;
}
