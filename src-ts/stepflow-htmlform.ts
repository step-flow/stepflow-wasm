import { Flow } from './flow';
import * as stepflow from './stepflow-wrapper';

type ON_START_WITH = (formSession: HtmlFormSession, startWith: stepflow.ActionStartWith) => void;
type ON_CANNOT_FULFILL = (formSession: HtmlFormSession, cannotFulfill: stepflow.ActionCannotFulfill) => void;
type ON_FINISH = (formSession: HtmlFormSession, statedata: stepflow.StateData) => void;

export class HtmlFormSession {
  private _form: HtmlForm;
  private _flowAnchorElement: HTMLElement;
  private _session: stepflow.Session;

  private _onstartwith: ON_START_WITH | null;
  private _oncannotfulfill: ON_CANNOT_FULFILL | null;
  private _onfinish: ON_FINISH | null;

  constructor(flow: Flow, formElement: HTMLFormElement, flowAnchorElement: HTMLElement) {
    // set initial state
    this._onstartwith = this._oncannotfulfill = this._onfinish = null;

    // create the session
    this._session = new stepflow.Session(flow);

    // setup the form
    this._form = new HtmlForm(formElement);
    this._form.onsubmit = (statedata: stepflow.StateData) => {
      this.advance(statedata);
    }

    // make sure the anchor is the form or a child of it
    if (!this._form.isOrHasChild(flowAnchorElement)) {
      throw new Error("flowAnchorElement not a part of the form");
    }
    this._flowAnchorElement = flowAnchorElement;
  }

  set onstartwith(onstartwith: (formSession: HtmlFormSession, startWith: stepflow.ActionStartWith) => void) {
    this._onstartwith = onstartwith;
  }
  set oncannotfulfill(oncannotfulfill: (formSession: HtmlFormSession, cannotFulfill: stepflow.ActionCannotFulfill) => void) {
    this._oncannotfulfill = oncannotfulfill;
  }
  set onfinish(onfinish: (formSession: HtmlFormSession, statedata: stepflow.StateData) => void) {
    this._onfinish = onfinish;
  }

  advance(input?: stepflow.StateData) {
    const result = this._session.advance(input);
    switch (result.blockedOn) {
      case "ActionStartWith":
        this.handleStartWith(result);
        break;
      case "ActionCannotFulfill":
        this.handleCannotFulfill(result);
        break;
      case "FinishedAdvancing":
        this.handleFinishedAdvancing(result);
        break;
    }
  }

  private handleStartWith(startWith: stepflow.ActionStartWith) {
    switch (startWith.actionType) {
      case "HtmlForm":
        this._flowAnchorElement.innerHTML = startWith.startWith
        break;
    }
    if (this._onstartwith) {
      this._onstartwith(this, startWith);
    }
  }

  private handleCannotFulfill(cannotFulfill: stepflow.ActionCannotFulfill) {
    if (this._oncannotfulfill) {
      this._oncannotfulfill(this, cannotFulfill);
    }
  }

  private handleFinishedAdvancing(finishedAdvancing: stepflow.ActionFinishedAdvancing) {
    if (this._onfinish) {
      this._onfinish(this, this._session.statedata());
    }
  }
}

type ON_SUBMIT = (statedata: stepflow.StateData) => void;

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

  set onsubmit(onsubmit: (statedata: stepflow.StateData) => void) {
    this._onsubmit = onsubmit;
  }

  into_statedata(): stepflow.StateData {
    // get all the values from the formdata
    const formData = new FormData(this.formElement);
    const statedata = stepflow.toStateData(formData);
  
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
    for (let elem: HTMLElement | null = e; elem = elem.parentElement; elem) {
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

export function attachFlowToHtmlForm(flow: Flow, formElement: HTMLFormElement, flowAnchorElement: HTMLElement): HtmlFormSession {
  const htmlFormSession = new HtmlFormSession(flow, formElement, flowAnchorElement);

  (formElement as StepFlowFormElement).htmlFormSession = htmlFormSession;
  if (!isStepFlowFormElement(formElement)) {
    throw new Error("Unable to attach StepFlow session to form element");
  }

  return htmlFormSession;
}