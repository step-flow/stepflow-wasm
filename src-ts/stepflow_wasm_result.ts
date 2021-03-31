type ResultOk = {
  ok: any;
}

export type ResultErr = {
  err: string;
}

export function isResultErr(r: any): r is ResultErr {
  if (typeof r != "object") {
    return false;
  }

  return typeof r["err"] == "string";
}

export function isResultOk(r: any): r is ResultOk {
  if (typeof r != "object") {
    return false;
  }

  return r["ok"] !== undefined;
}

export class ResultError extends Error {
  constructor(wasmResultErr: ResultErr) {
    super(wasmResultErr.err);
    this.name = "ResultError";
  }
}

export function isResultError(o: any): o is ResultError {
  return o instanceof ResultError;
}