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
    parameters: ["pointer"],
    result: "i32",
    nonblocking: true,
  },
  "deno_shell_exec_sync": {
    parameters: ["pointer"],
    result: "i32",
  },
});

export function exec(script: string): Promise<number> {
  return dylib.symbols.deno_shell_exec(encode(script));
}

export function execSync(script: string): number {
  return dylib.symbols.deno_shell_exec_sync(encode(script));
}

function encode(value: string): Uint8Array {
  return encoder.encode(value + "\0");
}
