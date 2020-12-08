import {intersectionOfLists} from "./util";
import assert from 'assert'

describe('util', () => {
  it('should return intersection of lists', () => {
    assert.deepStrictEqual(intersectionOfLists([[22, 22, 3], [2, 3]]), [3])
    assert.deepStrictEqual(intersectionOfLists([[22, 3, 4, 7], [2, 3, 4]]), [3, 4])
  });
});
