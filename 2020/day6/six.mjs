import {fileToString, intersectionOfLists, sumReducer} from "../util";

export const readInputFromData = content => {
  let groupContent = content.split('\n\n');
  return groupContent.map(gc => {
    let lines = gc.split('\n');
    return lines.filter(line => Boolean(line)).map(line => [...line])
  })
};

export const readInputFromFile = async () => {
  let content = await fileToString('data6');
  return readInputFromData(content);
}

export const getUniqueAssertiveAnswersFromEachGroup = (groups) => {
  return groups.map(group =>
      new Set(group.reduce(((agg, answerFromAPerson) => [...agg, ...answerFromAPerson]), [])).size
  )
}

export const getTotalUniqueAssertiveAnswersFromEachGroup = (groups) => {
  return getUniqueAssertiveAnswersFromEachGroup(groups).reduce(sumReducer())
}

export const getGroupwiseCountOfAnswersWhereEveryoneFromThatGroupWereAssertive = (groups) => {
  return groups.map(group => intersectionOfLists(group).length)
}

(async () => {
  let input = await readInputFromFile();
  console.log(getTotalUniqueAssertiveAnswersFromEachGroup(input));

  console.log(getGroupwiseCountOfAnswersWhereEveryoneFromThatGroupWereAssertive(input).reduce(sumReducer))
})()
