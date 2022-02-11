// const code = await Deno.readFile("./pkg/wasm_web_bg.wasm");
// const module = new WebAssembly.Module(code);
// const instance = new WebAssembly.Instance(module); // need imports object when using Web APIs
// const greet = instance.exports.greet as CallableFunction;
// greet();

import init, {greet} from "./pkg/wasm_web.js";
await init(Deno.readFile('./pkg/wasm_web_bg.wasm'));
greet();