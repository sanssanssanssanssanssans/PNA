export class BreakSignal extends Error {
  constructor() { super("break"); }
}
export class ContinueSignal extends Error {
  constructor() { super("continue"); }
}
