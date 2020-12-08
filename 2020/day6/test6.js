import {
  getGroupwiseCountOfAnswersWhereEveryoneFromThatGroupWereAssertive,
  getTotalUniqueAssertiveAnswersFromEachGroup,
  readInputFromData
} from "./six";
import assert from 'assert'
import {sumReducer} from "../util";

const testData = `
abc

a
b
c

ab
ac

a
a
a
a

b
`

describe('Customs declaration', () => {
  it('should get total count of unique assertive answers from each group', () => {
    let groups = readInputFromData(testData);
    let totalUniqueAssertiveAnswersFromEachGroup = getTotalUniqueAssertiveAnswersFromEachGroup(groups);

    assert.strictEqual(totalUniqueAssertiveAnswersFromEachGroup, 11);
  });

  it('should get total count of assertive answers from everyone each group', () => {
    let groups = readInputFromData(testData);
    let totalUniqueAssertiveAnswersFromEachGroup = getGroupwiseCountOfAnswersWhereEveryoneFromThatGroupWereAssertive(groups);

    console.log(totalUniqueAssertiveAnswersFromEachGroup)
    assert.strictEqual(totalUniqueAssertiveAnswersFromEachGroup.reduce(sumReducer), 6);
  });
});
