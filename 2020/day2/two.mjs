import {fileToLines} from "../util";
import {map, pipe, split} from 'ramda';

export const readInput = lines => {
  return lines.map(item => {
    let parts = item.split(' ');
    let [number1, number2] = pipe(split('-'), map(parseInt))(parts[0]);
    let character = parts[1][0];
    let password = parts[2];

    return {number1, number2, character, password}
  })
};

export const readInputFromFile = async () => {
  let lines = await fileToLines('data2');
  return readInput(lines);
}

export const getValidPasswords = (passwordsWithPolicy) => {
  return passwordsWithPolicy.filter(item => {
    let regExpMatchArray = item.password.match(new RegExp(item.character, "g"));
    if (regExpMatchArray) {
      let occurrence = regExpMatchArray.length
      return (occurrence <= item.number2) && (occurrence >= item.number1);
    }
  })
}

export const getValidPasswordsWithCorrectedPolicy = (passwordsWithPolicy) => {
  return passwordsWithPolicy.filter(item => {
    let one = item.password[item.number1 - 1] === item.character;
    let two = item.password[item.number2 - 1] === item.character;
    return Boolean(one ^ two);
  })
}


(async () => {
  let input = await readInputFromFile();
  console.log(getValidPasswords(input).length)

  input = await readInputFromFile();
  console.log(getValidPasswordsWithCorrectedPolicy(input).length)
})()

