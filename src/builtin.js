import crypto from "crypto";

export const builtins = {
  len: (x) => String(x).length,
  int: (x) => parseInt(Number(x)),
  str: (x) => String(x),
  bool: (x) => !!x,
  not: (x) => !x,

  random: (a, b) => {
    let lo = Math.min(Number(a), Number(b));
    let hi = Math.max(Number(a), Number(b));
    // secure-ish randint
    const span = hi - lo + 1;
    const rnd = Number.parseInt(crypto.randomBytes(4).readUInt32BE(0)) % span;
    return lo + rnd;
  },
  randint: (a, b) => {
    let lo = Math.min(Number(a), Number(b));
    let hi = Math.max(Number(a), Number(b));
    const span = hi - lo + 1;
    const rnd = Number.parseInt(crypto.randomBytes(4).readUInt32BE(0)) % span;
    return lo + rnd;
  },

  sleep: async (sec) => {
    const ms = Math.floor(Number(sec) * 1000);
    await new Promise((r) => setTimeout(r, ms));
    return true;
  },

  inlist: (val, csv) => String(csv).split(",").map(x=>x.trim()).includes(String(val)),
  contains: (s, sub) => String(s).includes(String(sub)),
  startswith: (s, prefix) => String(s).startsWith(String(prefix)),
  endswith: (s, suffix) => String(s).endsWith(String(suffix)),
  choice: (...args) => args[Math.floor(Math.random()*args.length)],
  capitalize: (s) => {
    s = String(s);
    return s.length? s[0].toUpperCase()+s.slice(1).toLowerCase(): s;
  },
  slice: (s, a, b) => String(s).slice(parseInt(a), parseInt(b)),
};
