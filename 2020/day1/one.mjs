import {fileToNumbers} from "../util";

export const findTwoNumbersSumming = (list, sum) => {
  let number1 = list.find(item => list.includes(sum - item));
  if (number1) return {number1, number2: sum - number1}
}

export const findThreeNumbersSumming = (list, sum) => {
  for (let i = 0; i < list.length; i++) {
    let numbers = findTwoNumbersSumming(list.slice(i, list.length), sum - list[i])
    if (numbers) return {...numbers, number3: list[i]}
  }
}

(async () => {
  let input = await fileToNumbers('data1');
  let numbers = findTwoNumbersSumming(input, 2020);
  console.log("Part 1", numbers.number1 * numbers.number2);

  numbers = findThreeNumbersSumming(input, 2020);
  console.log("Part 2", numbers.number1 * numbers.number2 * numbers.number3);
})()
