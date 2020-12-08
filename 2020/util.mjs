import fs from "fs";
import {filter, forEach, map, pipe, split} from "ramda";


export const fileToNumbers = async (file) => {
  let data = await fs.promises.readFile(file);
  return linesToNumbers(data.toString());
}

export const fileToLines = async (file) => {
  let data = await fs.promises.readFile(file);
  return pipe(split('\n'), filter(Boolean))(data.toString())
}

export const fileToString = async (file) => {
  return (await fs.promises.readFile(file)).toString();
}

export const linesToNumbers = (lines) => {
  return pipe(split('\n'), map(parseInt), filter(Boolean))(lines)
}

export const isBetween = (n, min, max) => {
  return (n <= max) && (n >= min);
}

export const intersectionOfLists = (lists) => {
  return lists.reduce((agg, list) => {
    let intersection = []
    list.forEach(item => {
      if (agg.includes(item)) {
        intersection.push(item);
      }
    })
    return intersection;
  })
}


export const sumReducer = (agg, item) => item + agg;
