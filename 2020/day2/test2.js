import {getValidPasswords, getValidPasswordsWithCorrectedPolicy, readInput} from "./two";
import assert from 'assert'
const testData = `1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc`

describe('password validator', () => {
  let input = readInput(testData.split('\n'));

  it('should get all valid passwords', () => {
    let validPasswords = getValidPasswords(input);
    assert.strictEqual(validPasswords.length, 2)
  });


  it('should get all valid passwords with corrected policy', () => {
    let validPasswords = getValidPasswordsWithCorrectedPolicy(input);
    assert.strictEqual(validPasswords.length, 1)
  });
});
