import {findThreeNumbersSumming, findTwoNumbersSumming} from "./one";
import {linesToNumbers} from "../util";
import assert from 'assert';

let testData = `1721
979
366
299
675
1456`

describe('findEntriesSumming', () => {

  it('should find two numbers summing to 2020', () => {
    let result = findTwoNumbersSumming(linesToNumbers(testData), 2020);

    assert.strictEqual(514579, result.number1 * result.number2)
  });

  it('should find three numbers summing to 2020', () => {
    let result = findThreeNumbersSumming(linesToNumbers(testData), 2020);

    assert.strictEqual(241861950, result.number1 * result.number2 * result.number3)
  });
});
