import {fileToLines} from "../util";

export const readForestMapFromFile = async () => {
  let lines = await fileToLines('data3');
  return readForestMap(lines);
}

export const readForestMap = (input) => {
  return input.map(line => [...line].map(character => character === '#' ? 1 : 0))
}

export const getNumberOfTreesEnRoute = (forestMap, slope) => {
  let widthOfForestPattern = forestMap[0].length;
  let currentCoOrdinates = {x: 0, y: 0};
  let treesEncountered = 0;
  while (currentCoOrdinates.y < forestMap.length) {
    if (forestMap[currentCoOrdinates.y][currentCoOrdinates.x % widthOfForestPattern] === 1)
      treesEncountered++
    currentCoOrdinates.x = currentCoOrdinates.x + slope.x;
    currentCoOrdinates.y = currentCoOrdinates.y + slope.y;
  }
  return treesEncountered;
}


export const computeProductOfAllPossibleNumberOfTree = (forestMap, slopes) =>
    slopes.reduce((aggregate, slope) => {
      return aggregate * getNumberOfTreesEnRoute(forestMap, slope)
    }, 1);

(async () => {
  let forestMap = await readForestMapFromFile();
  console.log(getNumberOfTreesEnRoute(forestMap, {x: 3, y: 1}))


  let slopes = [{x: 1, y: 1}, {x: 3, y: 1}, {x: 5, y: 1}, {x: 7, y: 1}, {x: 1, y: 2},]

  let part2result = computeProductOfAllPossibleNumberOfTree(forestMap, slopes);

  console.log(part2result)
})()
