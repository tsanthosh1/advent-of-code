import {fileToLines} from "../util";

export const getAllSeatIds = (seatSpecifications) => {
  return seatSpecifications.map(spec => getSeatIdFromSpecification(spec, 128, 8));
}

export const getMaxSeatId = (seatSpecifications) => {
  return Math.max(...getAllSeatIds(seatSpecifications))
}

const lowerHalf = (start, end) => Math.floor((end - start) / 2) + start;
const upperHalf = (start, end) => Math.ceil((end - start) / 2) + start;

const validateResult = ({startRow, endRow, startColumn, endColumn}) => startRow !== endRow && startColumn !== endColumn;

export const getSeatIdFromSpecification = (seatSpecification, noOfRows, noOfColumns) => {

  let result = [...seatSpecification].reduce((agg, spec) => {
    let {endRow, startRow, endColumn, startColumn} = agg;

    switch (spec) {
      case 'F':
        return {...agg, endRow: lowerHalf(startRow, endRow)}
      case 'B':
        return {...agg, startRow: upperHalf(startRow, endRow)}
      case 'L':
        return {...agg, endColumn: lowerHalf(startColumn, endColumn)}
      case 'R':
        return {...agg, startColumn: upperHalf(startColumn, endColumn)}
      default:
        return agg;
    }
  }, {startRow: 0, endRow: noOfRows - 1, startColumn: 0, endColumn: noOfColumns - 1});

  if (validateResult(result)) throw new Error("Not able to decide")

  return (result.startRow * 8) + result.startColumn;

}

const sum = (array) => array.reduce(((sum, num) => sum + num), 0);

const sumOfNNumbers = (n) => (n * (n + 1)) / 2;

(async () => {
  let input = await fileToLines('data5')

  console.log("Max seat ID", getMaxSeatId(input))

  let allSeatIds = getAllSeatIds(input);
  let min = Math.min(...allSeatIds);
  let max = Math.max(...allSeatIds);
  console.log("Your seat ID", sumOfNNumbers(max) - sumOfNNumbers(min - 1) - sum(allSeatIds))

})()
