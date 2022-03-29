import { parse, Args } from "https://deno.land/std@0.130.0/flags/mod.ts";
import init, {greet} from "./target/generated/wasm_web.js";

async function main(args: Args): Promise<void> {
    await init(Deno.readFile('./target/generated/wasm_web_bg.wasm'));

    if (args._.includes("start")) {
        return;
    }

    greet();
}

const parsedArgs = parse(Deno.args);
await main(parsedArgs);
