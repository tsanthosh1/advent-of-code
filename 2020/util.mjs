import fs from "fs";
import {filter, map, pipe, split} from "ramda";


export const fileToNumbers = async (file) => {
  let data = await fs.promises.readFile(file);
  return linesToNumbers(data.toString());
}

export const fileToLines = async (file) => {
  let data = await fs.promises.readFile(file);
  return pipe(split('\n'), filter(Boolean))(data.toString())
}

export const linesToNumbers = (lines) => {
  return pipe(split('\n'), map(parseInt), filter(Boolean))(lines)
}
