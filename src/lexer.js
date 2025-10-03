export const TT = {
  NUM: "NUM",
  STR: "STR",
  BOOL: "BOOL",
  OP: "OP",
  LP: "LP",
  RP: "RP",
  END: "END",
};

export class Lexer {
  constructor(s){ this.a = s; this.i = 0; }
  _peek(){ return this.a[this.i]; }
  _eof(){ return this.i >= this.a.length; }
  _isSpace(c){ return /\s/.test(c); }

  next() {
    while (!this._eof() && this._isSpace(this._peek())) this.i++;
    if (this._eof()) return {t:TT.END};
    const c = this._peek();

    if (c === "("){ this.i++; return {t:TT.LP}; }
    if (c === ")"){ this.i++; return {t:TT.RP}; }

    // string "..."
    if (c === '"' || c === "'"){
      const quote = c; this.i++;
      let buf = "", esc = false;
      while (!this._eof()){
        const d = this.a[this.i++];
        if (esc){ if (d==="n") buf+="\n"; else buf+=d; esc=false; continue; }
        if (d === "\\"){ esc = true; continue; }
        if (d === quote) break;
        buf += d;
      }
      return {t:TT.STR, v:buf};
    }

    // number
    if (/[0-9\-]/.test(c)){
      let j = this.i;
      let dot = false;
      if (this.a[j] === "-") j++;
      while (j<this.a.length && (/[0-9]/.test(this.a[j]) || (!dot && this.a[j]==="."))){
        if (this.a[j]===".") dot=true;
        j++;
      }
      const num = this.a.slice(this.i, j);
      this.i = j;
      if (/^-?\d+(\.\d+)?$/.test(num)) return {t:TT.NUM, v:Number(num)};
      return {t:TT.STR, v:num};
    }

    // operators
    const ops = ["==","!=", "<=", ">=", "&&","||","+","-","*","/","%","<",">","!"];
    for (const op of ops){
      if (this.a.startsWith(op, this.i)) {
        this.i += op.length;
        return {t:TT.OP, v:op};
      }
    }

    let j = this.i;
    while (j<this.a.length && /[A-Za-z0-9_\.]/.test(this.a[j])) j++;
    const id = this.a.slice(this.i, j);
    this.i = j;
    if (id === "true")  return {t:TT.BOOL, v:true};
    if (id === "false") return {t:TT.BOOL, v:false};
    return {t:TT.STR, v:id};
  }
}