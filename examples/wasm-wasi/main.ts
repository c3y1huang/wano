import Context from "https://deno.land/std@0.125.0/wasi/snapshot_preview1.ts";

const context = new Context({
    args: Deno.args,
    env: Deno.env.toObject(),
});

const binary = await Deno.readFile("./target/wasm32-wasi/debug/wasm-wasi.wasm");
const module = await WebAssembly.compile(binary);
const instance = await WebAssembly.instantiate(module, {
    "wasi_snapshot_preview1": context.exports,
});

console.log(instance.exports);

context.start(instance);
const greet = instance.exports.greet as CallableFunction;
greet();
