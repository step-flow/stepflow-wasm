import * as stepflow from '../pkg/index';

test('basic session', () => {
  const session = new stepflow.Session({
		steps: {
			"$root": { outputs: ["a"]}
		},
		actions: {
			"$all": { type: "htmlForm" }
		}
	});

  // run initial step
  let result = session.advance();
  expect(result.blockedOn).toBe("ActionStartWith");
  if (result.blockedOn == "ActionStartWith") {
    expect(result.actionType).toBe("HtmlForm");
  }

  // fulfill output
  const statedata: StateData = { a: "hi"};
  result = session.advance(statedata);
  expect(result.blockedOn).toBe("FinishedAdvancing");
});