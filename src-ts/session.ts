import { StepFlowWasm, SessionId, AdvanceBlockedOn, StateData } from './stepflow_wasm.js';
import { Flow } from "./flow.js";

/// Session
///
/// Ergonomic class to use a StepFlow Sesssion.
export class Session {
  private wasm: StepFlowWasm;
  private sessionId: SessionId;

  constructor(flow: Flow, wasm: StepFlowWasm) {
    this.wasm = wasm;
    this.sessionId = wasm.createSession(JSON.stringify(flow));
  }

  advance(input?: StateData | null): AdvanceBlockedOn {
    return this.wasm.advanceSession(this.sessionId, input ? JSON.stringify(input) : null);
  }

  statedata() : StateData {
    return this.wasm.getStateData(this.sessionId);
  }
}
