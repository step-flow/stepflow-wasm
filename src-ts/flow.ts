export type FlowVar = "String" | "Email" | "True" | "Bool";

export type FlowStep = {
  inputs?: string[],
  outputs: string[],
  substeps?: [string],
};

export type StringTemplateAction = {
  type: "stringTemplate",
  template: String,
  escape_for: "html" | "uri",
}

export type SetDataAction = {
  type: "setData",
  data: { [varname: string]: string },    // varname -> value
  after_attempt: number,
}

export type HtmlFormAction = {
  type: "htmlForm"
  string_html?: string
  email_html?: string
  bool_html?: string
  prefix_html?: string, // ie. label before each input field
  wrap_tag?: string, // ie. wrap entire element in a <div></div>      
}

export type FlowAction = StringTemplateAction | SetDataAction | HtmlFormAction;

export type Flow = {
  vars?: {
    [varName: string]: FlowVar,
  };
  steps: {
    "$root": FlowStep,
    [stepName: string]: FlowStep,
  },
  actions: {
    [stepName: string]: FlowAction,
  }
};
