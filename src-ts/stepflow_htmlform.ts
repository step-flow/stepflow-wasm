import { Flow } from './flow.js';
import { StepFlowWasm, StateData, AdvanceBlockedOnStartWith, AdvanceBlockedOnCannotFulfill, AdvanceBlockedOnFinishedAdvancing } from './stepflow_wasm.js';
import { ResultError, isResultError } from './stepflow_wasm_result.js';
import { Session } from './session.js';

type ON_START_WITH = (formSession: HtmlFormSession, startWith: AdvanceBlockedOnStartWith) => void;
type ON_CANNOT_FULFILL = (formSession: HtmlFormSession, cannotFulfill: AdvanceBlockedOnCannotFulfill) => void;
type ON_FINISH = (formSession: HtmlFormSession, statedata: StateData) => void;
type ON_FAIL_ADVANCE = (formSession: HtmlFormSession, err: ResultError) => void;

export class HtmlFormSession {
  private _form: HtmlForm;
  private _flowAnchorElement: HTMLElement;
  private _session: Session;
  private _isFinished;

  private _onStartWith: ON_START_WITH | null;
  private _onCannotFulfill: ON_CANNOT_FULFILL | null;
  private _onFinish: ON_FINISH | null;
  private _onFailAdvance: ON_FAIL_ADVANCE | null;

  constructor(flow: Flow, wasm: StepFlowWasm, formElement: HTMLFormElement, flowAnchorElement?: HTMLElement) {
    // set initial state
    this._onStartWith = this._onCannotFulfill = this._onFinish = this._onFailAdvance = null;
    this._isFinished = false;

    // create the session
    this._session = new Session(flow, wasm);

    // setup the form
    this._form = new HtmlForm(formElement);
    this._form.onsubmit = (statedata: StateData) => {
      this.advance(statedata);
    }

    // make sure the anchor is the form or a child of it
    if (!flowAnchorElement) {
      flowAnchorElement = formElement;
    } else if (!this._form.isOrHasChild(flowAnchorElement)) {
      throw new Error("flowAnchorElement not a part of the form");
    }
    this._flowAnchorElement = flowAnchorElement;
  }

  set onStartWith(onStartWith: ON_START_WITH) {
    this._onStartWith = onStartWith;
  }
  set onCannotFulfill(onCannotFulfill: ON_CANNOT_FULFILL) {
    this._onCannotFulfill = onCannotFulfill;
  }
  set onFinish(onFinish: ON_FINISH) {
    this._onFinish = onFinish;
  }
  set onFailAdvance(onFailAdvance: ON_FAIL_ADVANCE) {
    this._onFailAdvance = onFailAdvance;
  }

  advance(input?: StateData) {
    let result;
    if (!this._isFinished) {
      try {
        result = this._session.advance(input);
      } catch (e) {
        if (isResultError(e) && this._onFailAdvance) {
          return this._onFailAdvance(this, e);
        } else {
          throw e;
        }
      }
    } else {
      const finishedResult: AdvanceBlockedOnFinishedAdvancing = {
        blockedOn: "FinishedAdvancing",
      };
      result = finishedResult;
    }

    switch (result.blockedOn) {
      case "StartWith":
        this.handleStartWith(result);
        break;
      case "CannotFulfill":
        this.handleCannotFulfill(result);
        break;
      case "FinishedAdvancing":
        this.handleFinishedAdvancing(result);
        break;
    }
  }

  private handleStartWith(startWith: AdvanceBlockedOnStartWith) {
    switch (startWith.actionType) {
      case "HtmlForm":
        this._flowAnchorElement.innerHTML = startWith.startWith
        break;
    }
    if (this._onStartWith) {
      this._onStartWith(this, startWith);
    }
  }

  private handleCannotFulfill(cannotFulfill: AdvanceBlockedOnCannotFulfill) {
    if (this._onCannotFulfill) {
      this._onCannotFulfill(this, cannotFulfill);
    }
  }

  private handleFinishedAdvancing(finishedAdvancing: AdvanceBlockedOnFinishedAdvancing) {
    this._isFinished = true;
    if (this._onFinish) {
      this._onFinish(this, this._session.statedata());
    }
  }
}

type ON_SUBMIT = (statedata: StateData) => void;

class HtmlForm {
  private _formElement: HTMLFormElement;
  private _onsubmit: ON_SUBMIT | null;

  constructor(formElement: HTMLFormElement) {
    this._formElement = formElement;
    this._onsubmit = null;
    this._formElement.onsubmit = (evt) => {
      if (this._onsubmit) {
        const statedata = this.into_statedata();
        this._onsubmit(statedata);
        return false;
      }
    };
  }

  get formElement(): HTMLFormElement {
    return this._formElement;
  }

  set onsubmit(onsubmit: (statedata: StateData) => void) {
    this._onsubmit = onsubmit;
  }

  into_statedata(): StateData {
    // get all the values from the formdata
    const formData = new FormData(this.formElement);
    const statedata = toStateData(formData);
  
    // checkboxes are absent value if not checked so look for unchecked values
    this.foreachTagNameAndAttribute("input", "type", "checkbox", (checkbox: Element) => {
      let name = checkbox.getAttribute("name");
      if (!name) {
        name = checkbox.getAttribute("id");
      }
      if (!name) {
        return;
      }
    
      if (statedata[name]) {
        statedata[name] = "true";
      } else {
        statedata[name] = "false";
      }
    });
  
    return statedata;
  }

  isOrHasChild(e: HTMLElement): boolean {
    for (let elem: HTMLElement | null = e; elem; elem = elem.parentElement) {
      if (elem === this._formElement) {
        return true;
      }
    }
    return false;
  }

  private foreachTagNameAndAttribute(tagName: string, attrName: string, attrValue: string, cb: (e: Element) => void) {
    const elements = this.formElement.getElementsByTagName(tagName);
    for (let i = 0; i < elements.length; i++) {
      const element = elements[i];
      const attribute = element.getAttribute(attrName);
      if (attribute == attrValue) {
        cb(element);
      }
    }
  }
}

export interface StepFlowFormElement extends HTMLFormElement {
  htmlFormSession: HtmlFormSession;
}

export function isStepFlowFormElement(e: HTMLElement): e is StepFlowFormElement {
  return (e as StepFlowFormElement).htmlFormSession !== undefined;
}

export function getHtmlFormSession(e: HTMLFormElement): HtmlFormSession | null {
  if (!isStepFlowFormElement(e)) {
    return null;
  }
  return e.htmlFormSession;
}

export function attachFlowToHtmlForm(flow: Flow, wasm: StepFlowWasm, formElement: HTMLFormElement, flowAnchorElement: HTMLElement): HtmlFormSession {
  const htmlFormSession = new HtmlFormSession(flow, wasm, formElement, flowAnchorElement);

  (formElement as StepFlowFormElement).htmlFormSession = htmlFormSession;
  if (!isStepFlowFormElement(formElement)) {
    throw new Error("Unable to attach StepFlow session to form element");
  }

  return htmlFormSession;
}

function toStateData(kvIter: any): StateData {
  const result: StateData = {}
  for (const o of kvIter) {
    const [k, v] = o;
    result[k] = v;
  }
  return result;
}
