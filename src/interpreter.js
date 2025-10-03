import readline from "readline/promises";
import { stdin as input, stdout as output } from "process";
import { builtins } from "./builtin.js";
import { ExprParser } from "./parser.js";
import { BreakSignal, ContinueSignal } from "./errors.js";

export class Interpreter {
  constructor() {
    this.variables = {};
    this.rl = readline.createInterface({ input, output });
    this.parser = new ExprParser((id) => this._resolveIdentifier(id));
  }

  _resolveIdentifier(tok) {
    if (/^[A-Za-z_][A-Za-z0-9_]*\.[A-Za-z_][A-Za-z0-9_]*$/.test(tok)) {
      const [v, k] = tok.split(".");
      if (this.variables[v] && typeof this.variables[v] === "object" && this.variables[v] !== null && Object.prototype.hasOwnProperty.call(this.variables[v], k)) {
        const val = this.variables[v][k];
        return val;
      }
      return tok; // 미해결
    }
    if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(tok)) {
      if (Object.prototype.hasOwnProperty.call(this.variables, tok)) {
        const val = this.variables[tok];
        return val;
      }
    }
    return null;
  }

  _normalizeLines(codeOrLines) {
    const raw = Array.isArray(codeOrLines) ? codeOrLines.join("\n") : String(codeOrLines);
    return raw
      .split("\n")
      .map((l) => l.trim())
      .filter((l) => l.length && !l.startsWith("#") && !l.startsWith("//"));
  }

  async close() {
    await this.rl.close();
  }

  async evalExpr(expr) {
    const res = this.parser.parse(expr);
    if (res && typeof res.then === "function") return await res;
    return res;
  }

  async execute(codeOrLines) {
    const lines = this._normalizeLines(codeOrLines);
    let i = 0;

    while (i < lines.length) {
      const line = lines[i];

      if (/^\w+:\s*{$/.test(line)) {
        const varName = line.split(":")[0].trim();
        const obj = {};
        i++;
        while (i < lines.length && lines[i] !== "}") {
          const kv = lines[i];
          const m = /^(\w+)\s*:\s*(.+?)(,)?$/.exec(kv);
          if (!m) { i++; continue; }
          const key = m[1];
          let val = m[2].trim();
          try {
            val = await this.evalExpr(val);
          } catch (e) {
            console.log(`[ObjectParseError] ${e?.message ?? e}`);
          }
          obj[key] = val;
          i++;
        }
        this.variables[varName] = obj;
      }

      // --- var.prop : expr ---
      else if (/^\w+\.\w+\s*:/.test(line)) {
        const [lhs, rhsRaw] = line.split(":", 1 + 1);
        const [v, k] = lhs.trim().split(".");
        const rhs = rhsRaw.trim().replace(/,$/, "");
        try {
          const result = await this.evalExpr(rhs);
          if (!this.variables[v] || typeof this.variables[v] !== "object") {
            this.variables[v] = {};
          }
          this.variables[v][k] = result;
        } catch (e) {
          console.log(`[AssignError] ${e?.message ?? e}`);
        }
      }

      // --- var : expr  (문법 확장) ---
      else if (/^\w+\s*:/.test(line) && !/^\w+:\s*{$/.test(line)) {
        const [lhs, rhsRaw] = line.split(":", 1 + 1);
        const v = lhs.trim();
        const rhs = rhsRaw.trim().replace(/,$/, "");
        try {
          const result = await this.evalExpr(rhs);
          this.variables[v] = result;
        } catch (e) {
          console.log(`[AssignError] ${e?.message ?? e}`);
        }
      }

      // --- log expr ---
      else if (line.startsWith("log ")) {
        const expr = line.slice(4).trim();
        try {
          const res = await this.evalExpr(expr);
          console.log(res);
        } catch (e) {
          console.log(`[LogError] ${e?.message ?? e}`);
        }
      }

      // --- cond (expr) -> { ... } [else -> { ... }] end 
      else if (line.startsWith("cond")) {
        const m = /^cond\s*\((.*?)\)\s*->\s*{$/.exec(line);
        if (!m) { console.log(`[error] Invalid cond syntax: ${line}`); i++; continue; }
        const condExpr = m[1];

        const thenBlock = [];
        i++;
        while (i < lines.length && lines[i] !== "}" && lines[i] !== "end") {
          thenBlock.push(lines[i]);
          i++;
        }
        if (lines[i] === "}") i++; // consume '}'

        // optional else
        let elseBlock = null;
        if (i < lines.length && /^else\s*->\s*{$/.test(lines[i])) {
          elseBlock = [];
          i++;
          while (i < lines.length && lines[i] !== "}") {
            elseBlock.push(lines[i]); i++;
          }
          if (lines[i] === "}") i++;
        }

        // expect end
        if (i < lines.length && lines[i] === "end") {
          // ok
        } else {
          console.log(`[error] Missing 'end' for cond`);
        }

        let shouldRun = false;
        try {
          shouldRun = !!(await this.evalExpr(condExpr));
        } catch (e) {
          console.log(`[Conderror] ${e?.message ?? e}`);
        }
        if (shouldRun) {
          await this.execute(thenBlock);
        } else if (elseBlock) {
          await this.execute(elseBlock);
        }
      }

      // --- input "prompt" -> var(.prop)? ---
      else if (line.startsWith("input ")) {
        const m = /^input\s+"(.*?)"\s*->\s*(\w+)(?:\.(\w+))?$/.exec(line);
        if (m) {
          const prompt = m[1];
          const v = m[2];
          const p = m[3];

          let userInput = "";
          if (prompt === "NO") {
            userInput = await this.rl.question("");
          } else {
            userInput = await this.rl.question(prompt + " ");
          }

          if (/^-?\d+$/.test(userInput)) userInput = parseInt(userInput, 10);
          else if (/^-?\d+\.\d+$/.test(userInput)) userInput = parseFloat(userInput);

          if (p) {
            if (!this.variables[v] || typeof this.variables[v] !== "object") this.variables[v] = {};
            this.variables[v][p] = userInput;
          } else {
            this.variables[v] = userInput;
          }
        } else {
          console.log(`[InputError] Invalid input syntax: ${line}`);
        }
      }

      // --- loop (expr) -> { ... } end ---
      else if (line.startsWith("loop")) {
        const m = /^loop\s*\((.*?)\)\s*->\s*{$/.exec(line);
        if (!m) { console.log(`[error] Invalid loop syntax: ${line}`); i++; continue; }
        const condExpr = m[1];
        const block = [];
        i++;
        while (i < lines.length && lines[i] !== "end") {
          block.push(lines[i]);
          i++;
        }
        try {
          while (await this.evalExpr(condExpr)) {
            try {
              await this.execute(block);
            } catch (e) {
              if (e instanceof ContinueSignal) continue;
              if (e instanceof BreakSignal) break;
              throw e;
            }
          }
        } catch (e) {
          console.log(`[LoopError] ${e?.message ?? e}`);
        }
      }

      else if (line === "break") {
        throw new BreakSignal();
      } else if (line === "continue") {
        throw new ContinueSignal();
      }

      i++;
    }
  }
}
