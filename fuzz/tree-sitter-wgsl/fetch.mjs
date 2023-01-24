import { writeFile } from "fs/promises"

let val = await fetch("https://gpuweb.github.io/gpuweb/wgsl/grammar/grammar.js")

writeFile("./grammar.js", Buffer.from(await (await val.blob()).arrayBuffer()))
