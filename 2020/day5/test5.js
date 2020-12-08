import {getSeatIdFromSpecification} from "./five";
import assert from 'assert'

describe('Find Seat ID', () => {
  it('should get seat ID from specification', () => {
    assert.strictEqual(getSeatIdFromSpecification("FBFBBFFRLR", 128, 8), 357)
    assert.strictEqual(getSeatIdFromSpecification("BFFFBBFRRR", 128, 8), 567)
    assert.strictEqual(getSeatIdFromSpecification("FFFBBBFRRR", 128, 8), 119)
    assert.strictEqual(getSeatIdFromSpecification("BBFFBBFRLL", 128, 8), 820)
  });
});
