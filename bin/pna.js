import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { Interpreter } from "../src/interpreter.js";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function main() {
  const file = process.argv[2];
  if (!file) {
    console.error("Usage: pna <file.pna>");
    process.exit(1);
  }
  if (!file.endsWith(".pna")) {
    console.error("Only .pna files are supported.");
    process.exit(1);
  }
  if (!fs.existsSync(file)) {
    console.error(`${file} not found.`);
    process.exit(1);
  }
  const code = fs.readFileSync(file, "utf-8");
  const it = new Interpreter();
  await it.execute(code);
}

main().catch((e) => {
  console.error("[RuntimeError]", e?.message ?? e);
  process.exit(1);
});