import * as wasm from '../pkg/stepflow_wasm';
import { Flow } from "./flow";


export class Session {
  session: wasm.WebSession;

  constructor(flow: Flow) {
    this.session = wasm.createSession(JSON.stringify(flow));
  }

  advance(input?: StateData): AdvanceBlockedOn {
    const advanceResult = this.session.advance(input);
    const actionType = actionTypeFromWasmActionType(advanceResult.actionType);
    switch (advanceResult.blockedOn) {
      case wasm.WebAdvanceBlockedOnType.ActionStartWith:
        return {
          blockedOn: "ActionStartWith",
          action: advanceResult.action,
          actionType: actionType,
          startWith: advanceResult.start_with,
        }
      case wasm.WebAdvanceBlockedOnType.ActionCannotFulfill:
        return {
          blockedOn: "ActionCannotFulfill",
          action: advanceResult.action,
        }
      case wasm.WebAdvanceBlockedOnType.FinishedAdvancing:
        return {
          blockedOn: "FinishedAdvancing",
        }
      default:
        throw new Error(`Unexpected blockedOn type: ${advanceResult.blockedOn}`);
    }
  }

  statedata() : StateData {
    const statedata = this.session.statedata;
    const entries = statedata.entries();
    return toStateData(entries);
  }
}

export type StateData = {
  [k: string]: string,
};

export function toStateData(kvIter: any): StateData {
  const result: StateData = {}
  for (const o of kvIter) {
    const [k, v] = o;
    result[k] = v;
  }
  return result;
}

export type ActionType = "SetData" | "HtmlForm" | "Other";

function actionTypeFromWasmActionType(wasmActionType: number | undefined): ActionType {
  switch (wasmActionType) {
    case wasm.ActionType.SetData:
      return "SetData";
    case wasm.ActionType.HtmlForm:
      return "HtmlForm";
  }
  return "Other";
}

export type ActionStartWith = {
  blockedOn: "ActionStartWith",
  action: string,
  actionType?: ActionType
  startWith: any,
}

export type ActionCannotFulfill = {
  blockedOn: "ActionCannotFulfill",
  action: string,
  actionType?: ActionType
}

export type ActionFinishedAdvancing = {
  blockedOn: "FinishedAdvancing",
}

export type AdvanceBlockedOn = ActionStartWith | ActionCannotFulfill | ActionFinishedAdvancing;
