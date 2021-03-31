import { WasmMemory, Ptr} from "./wasm_mem.js";
import { isResultErr, isResultOk, ResultError} from "./stepflow_wasm_result.js";

export type SessionId = number;

type WasmExports = {
  createSession: (flowJsonStr: Ptr) => SessionId,
  advanceSession: (sessionId: SessionId, stepOutputJsonStr: Ptr | null) => Ptr,
  getStateData: (sessionId: SessionId) => Ptr,
  alloc: (size: number) => Ptr,
  dealloc: (ptr: Ptr, size: number) => void,
  deallocStr: (cstring_ptr: Ptr) => void,
};

/// StepFlowWasm
///
/// Gives access to the APIs in StepFlow while making the API
/// more manageable for JS/TS. 
export class StepFlowWasm {
  private exports: WasmExports;
  private memory: WasmMemory;
  
  constructor(wasmInstance: any) {
    this.exports = {
      createSession: wasmInstance.exports.createSession,
      advanceSession: wasmInstance.exports.advanceSession,
      getStateData: wasmInstance.exports.getStateData,
      alloc: wasmInstance.exports.alloc,
      dealloc: wasmInstance.exports.dealloc,
      deallocStr: wasmInstance.exports.dealloc_str,
    };
    this.memory = new WasmMemory(wasmInstance.exports.memory, this.exports.alloc, this.exports.dealloc, this.exports.deallocStr);
  }

  createSession(flowJsonStr: string): SessionId {
    return this.callExportedFnWithString(flowJsonStr, this.exports.createSession);
  }

  advanceSession(sessionId: SessionId, stepOutputJsonStr: string | null): AdvanceBlockedOn {
    let result;

    if (stepOutputJsonStr === null || stepOutputJsonStr === undefined) {
      result = this.callExportedFn(() => this.exports.advanceSession(sessionId, 0));
    } else {
      result = this.callExportedFnWithString(stepOutputJsonStr, (stepOutputPtr: Ptr) => this.exports.advanceSession(sessionId, stepOutputPtr));
    };

    return result;
  }

  getStateData(sessionId: SessionId): StateData {
    return this.callExportedFn(() => this.exports.getStateData(sessionId));
  }

  private execFnWithString(s: string, fn: any): any {
    // FUTURE: support multiple strings and do a single block allocation for all the strings
    const [strPtr, size] = this.memory.allocString(s);
    try {
      const result = fn(strPtr);
      this.memory.free(strPtr, size);
      return result;
  
    } catch(e) {
      this.memory.free(strPtr, size);
      throw e;
    }
  }
  
  private callExportedFn(fn: any) {
    const resultStrPtr = fn();
    const resultStr = this.memory.decodeCstr(resultStrPtr);
    const result: any = JSON.parse(resultStr);

    if (isResultErr(result)) {
      throw new ResultError(result);
    }
    else if (!isResultOk(result)) {
      throw new Error("Did not receive a valid StepFlowResult");
    }

    return result.ok;
  }
  
  private callExportedFnWithString(s: string, fn: any) {
    return this.callExportedFn(() => this.execFnWithString(s, fn));
  }
}

export function loadStepflowWasm(wasmSrc: string): Promise<StepFlowWasm> {
  return instantiateWasm(wasmSrc)
    .then(wasmInstance => new StepFlowWasm(wasmInstance));
}

async function instantiateWasm(wasmSrc: string) {
  return fetch(wasmSrc)
    .then(response => response.arrayBuffer())
    .then(bytes => WebAssembly.instantiate(bytes))
    .then(results => results.instance)
}

export type AdvanceBlockedOnStartWith = {
  blockedOn: "StartWith",
  startWith: any,
  actionName?: string,
  actionType?: "SetData" | "HtmlForm" | "UriStringTemplate" | "HtmlStringTemplate" | "Other",
};

export type AdvanceBlockedOnCannotFulfill = {
  blockedOn: "CannotFulfill",
}

export type AdvanceBlockedOnFinishedAdvancing = {
  blockedOn: "FinishedAdvancing",
}

export type AdvanceBlockedOn = AdvanceBlockedOnStartWith | AdvanceBlockedOnCannotFulfill | AdvanceBlockedOnFinishedAdvancing;

export type StateData = {
  [k: string]: any,
};
