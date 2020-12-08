import {computeProductOfAllPossibleNumberOfTree, getNumberOfTreesEnRoute, readForestMap} from "./three";
import assert from 'assert';

const testData = `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`

describe('getNumberOfTreeEnRoute', () => {
  let forestMap = readForestMap(testData.split('\n'));

  it('should calculate number of tree encountered', () => {

    let numberOfTrees = getNumberOfTreesEnRoute(forestMap, { x : 3, y: 1});

    assert.strictEqual(numberOfTrees, 7)
  });

  it('should compute product of all possible number of trees', () => {
    let slopes = [{x: 1, y: 1}, {x: 3, y: 1}, {x: 5, y: 1}, {x: 7, y: 1}, {x: 1, y: 2},]

    let result = computeProductOfAllPossibleNumberOfTree(forestMap, slopes);

    assert.strictEqual(result, 336)
  });
});
