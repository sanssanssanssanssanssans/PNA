import { Lexer, TT } from "./lexer.js";
import { builtins } from "./builtin.js";

export class ExprParser {
  constructor(resolveIdentifier) {
    this.resolveIdentifier = resolveIdentifier; // (bareword) => literal value (num/str/bool) or null
  }

  parse(expr) {
    const trimmed = expr.trim();
    const firstWord = trimmed.split(/\s+/,1)[0];
    if (builtins[firstWord]) {
      const argString = trimmed.slice(firstWord.length).trim();
      const argv = this._splitArgs(argString).map(a => this.parse(a));
      const fn = builtins[firstWord];
      const result = fn.length === argv.length ? fn(...argv) : fn(...argv);
      return result;
    }

    const L = new Lexer(this._substituteVariables(expr));
    return this._parseOr(L);
  }

  _splitArgs(s) {
    const out = [];
    let cur="", inStr=false, esc=false, quote=null;
    for (let i=0;i<s.length;i++){
      const c=s[i];
      if (inStr){
        if (esc){ cur+=c; esc=false; continue; }
        if (c==="\\"){ esc=true; cur+=c; continue; }
        if (c===quote){ inStr=false; cur+=c; quote=null; continue; }
        cur+=c;
      } else {
        if (c==='"'||c==="'"){ inStr=true; quote=c; cur+=c; continue; }
        if (/\s/.test(c)){
          if (cur.length){ out.push(cur); cur=""; }
        } else cur+=c;
      }
    }
    if (cur.length) out.push(cur);
    return out;
  }

  _substituteVariables(expr) {
    const tokens = expr.split(/(\s+|[()!<>=&|+\-*/%])/).filter(x=>x!==undefined);
    return tokens.map(tok=>{
      if (!tok) return tok;
      if (/^\s+$/.test(tok)) return tok;
      if (/^[()!<>=&|+\-*/%]$/.test(tok)) return tok;

      const val = this.resolveIdentifier(tok);
      if (val === null || val === undefined) return tok;

      if (typeof val === "string") return `"${val.replace(/\\/g,"\\\\").replace(/"/g,'\\"')}"`;
      if (typeof val === "boolean") return val ? "true" : "false";
      if (typeof val === "number" && Number.isFinite(val)) return String(val);
      return `"${String(val).replace(/\\/g,"\\\\").replace(/"/g,'\\"')}"`;
    }).join("");
  }

  // Pratt parser
  _parseExpr(L){ return this._parseOr(L); }

  _parsePrimary(L){
    const t = L.next();
    if (t.t===TT.NUM || t.t===TT.STR || t.t===TT.BOOL) return t.v;
    if (t.t===TT.LP){
      const v = this._parseExpr(L);
      const r = L.next(); // RP
      return v;
    }
    if (t.t===TT.OP && t.v==="!"){
      const a = this._parsePrimary(L);
      return !this._toBool(a);
    }
    // fallback
    return "";
  }

  _parseMul(L){
    let left = this._parsePrimary(L);
    while (true){
      const save = {i:L.i};
      const t = L.next();
      if (t.t===TT.OP && (t.v==="*"||t.v==="/"||t.v==="%")){
        const right = this._parsePrimary(L);
        if (t.v==="*") left = this._toNum(left) * this._toNum(right);
        else if (t.v==="/") left = this._toNum(right)===0 ? 0 : (this._toNum(left)/this._toNum(right));
        else left = this._toNum(left) % this._toNum(right);
      } else { L.i = save.i; break; }
    }
    return left;
  }

  _parseAdd(L){
    let left = this._parseMul(L);
    while (true){
      const save = {i:L.i};
      const t = L.next();
      if (t.t===TT.OP && (t.v==="+"||t.v==="-" )){
        const right = this._parseMul(L);
        if (t.v==="+"){
          if (typeof left==="string" || typeof right==="string") left = String(left) + String(right);
          else left = this._toNum(left) + this._toNum(right);
        } else left = this._toNum(left) - this._toNum(right);
      } else { L.i = save.i; break; }
    }
    return left;
  }

  _parseCmp(L){
    let left = this._parseAdd(L);
    while (true){
      const save = {i:L.i};
      const t = L.next();
      if (t.t===TT.OP && (t.v==="=="||t.v==="!="||t.v==="<"||t.v===">"||t.v==="<="||t.v===">=")){
        const right = this._parseAdd(L);
        let res = false;
        switch (t.v){
          case "==": res = String(left) === String(right); break;
          case "!=": res = String(left) !== String(right); break;
          case "<":  res = this._toNum(left) <  this._toNum(right); break;
          case ">":  res = this._toNum(left) >  this._toNum(right); break;
          case "<=": res = this._toNum(left) <= this._toNum(right); break;
          case ">=": res = this._toNum(left) >= this._toNum(right); break;
        }
        left = res;
      } else { L.i = save.i; break; }
    }
    return left;
  }

  _parseAnd(L){
    let left = this._parseCmp(L);
    while (true){
      const save = {i:L.i};
      const t = L.next();
      if (t.t===TT.OP && t.v==="&&"){
        const right = this._parseCmp(L);
        left = this._toBool(left) && this._toBool(right);
      } else { L.i = save.i; break; }
    }
    return left;
  }

  _parseOr(L){
    let left = this._parseAnd(L);
    while (true){
      const save = {i:L.i};
      const t = L.next();
      if (t.t===TT.OP && t.v==="||"){
        const right = this._parseAnd(L);
        left = this._toBool(left) || this._toBool(right);
      } else { L.i = save.i; break; }
    }
    return left;
  }

  _toNum(x){ return typeof x==="number" ? x : (typeof x==="boolean" ? (x?1:0) : Number(x)); }
  _toBool(x){ return typeof x==="boolean" ? x : (typeof x==="number" ? x!==0 : !!x); }
}
