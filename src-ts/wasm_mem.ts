type Memory = {
  buffer: Iterable<number>,
};
export type Ptr = number;
type Size = number;
type FnAlloc = (size: number) => Ptr;
type FnDealloc =  (ptr: Ptr, size: number) => void;
type FnDeallocStr =  (cstring_ptr: Ptr) => void;

/// WasmMemory
///
/// Basic memory management between the JS & WASM layer
export class WasmMemory {
  private memory: Memory;
  private fnAlloc: FnAlloc;
  private fnDealloc: FnDealloc;
  private fnDeallocStr: FnDeallocStr;

  constructor(memory: Memory, fnAlloc: FnAlloc, fnDealloc: FnDealloc, fnDeallocStr: FnDeallocStr) {
    this.memory = memory;
    this.fnAlloc = fnAlloc;
    this.fnDealloc = fnDealloc;
    this.fnDeallocStr = fnDeallocStr;
  }

  allocString(s: string): [Ptr, Size] {
    const utf8Encoder = new TextEncoder();
    let buffer = utf8Encoder.encode(s)
    let bufferSize = buffer.length;
    let ptr: Ptr = this.fnAlloc(bufferSize + 1);
  
    let memory = new Uint8Array(this.memory.buffer);
    for (let i = 0; i < bufferSize; i++) {
      memory[ptr + i] = buffer[i];
    }
    memory[ptr + bufferSize] = 0;
  
    return [ptr, bufferSize + 1];
  }

  free(ptr: Ptr, size: Size) {
    if (!ptr) {
      throw new Error("Cannot free a non-existent pointer");
    }
    this.fnDealloc(ptr, size);
  }

  freeCString(ptr: Ptr) {
    if (!ptr) {
      throw new Error("Cannot free a non-existent pointer to a CString");
    }
    this.fnDeallocStr(ptr);
  }

  decodeCstr(ptr: Ptr) {
    if (!ptr) {
      throw new Error("Cannot decode a non-existent pointer");
    }
    let m = new Uint8Array(this.memory.buffer);
    let s = "";
    for (let i = ptr; m[i]!==0; i++) {
      s += String.fromCharCode(m[i]);
    }
  
    // allow rust to reclaim the pointer.
    this.freeCString(ptr);
  
    return s;
  }
  
}