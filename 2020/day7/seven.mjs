import {fileToLines, sumReducer} from "../util";

export const readInputFromData = lines => {
  return lines.reduce((rulesAgg, line) => {
    let match = line.match(/(.*) bags contain (.*)/);
    let bag = match[1]

    let contents = match[2].split(',').reduce((bagContentAgg, item) => {
      if (item === "no other bags.") return {}
      let bags = item.match(/(\d+) (.*) bags?/);
      return {...bagContentAgg, [[bags[2]]]: bags[1]}
    }, {})

    return {...rulesAgg, [bag]: contents}
  }, {})
};

export const readInputFromFile = async () => {
  let lines = await fileToLines('data7');
  return readInputFromData(lines);
}

const checkBagContainingColor = (bagRules, bagColor, requiredColor, memoization) => {

  if (bagRules[bagColor][requiredColor]) {
    memoization[bagColor] = true;
    return true
  }
  else if (memoization[bagColor] !== undefined) {
    return memoization[bagColor]
  }
  else {
    let requiredColorFound = Object.keys(bagRules[bagColor]).reduce((agg, bag) => {
      return agg || checkBagContainingColor(bagRules, bag, requiredColor, memoization)
    }, false);

    memoization[bagColor] = requiredColorFound
    return requiredColorFound
  }

};

export const getBagsColorThatCanContainCertainColoredBagInIt = (bagRules, requiredColor) => {
  let memoization = {};
  return Object.keys(bagRules).filter(bag =>
      checkBagContainingColor(bagRules, bag, requiredColor, memoization)).length
}

const _getAllTheBagsInsideCertainColoredBag = (bagRules, color, memoization) => {
  if (memoization[color] !== undefined) {
    return memoization[color]
  }

  let count = 0
  let numbers = Object.values(bagRules[color]).map(x => parseInt(x));
  count += numbers.reduce(sumReducer, 0)
  count += Object.keys(bagRules[color]).reduce((sum, bagColor) =>
      sum + (bagRules[color][bagColor] * _getAllTheBagsInsideCertainColoredBag(bagRules, bagColor, memoization)), 0);
  memoization[color] = count;
  return count
}

export const getAllTheBagsInsideCertainColoredBag = (bagRules, color) => {
  return _getAllTheBagsInsideCertainColoredBag(bagRules, color, {}, 0);
}

(async () => {
  let input = await readInputFromFile();

  console.log(getBagsColorThatCanContainCertainColoredBagInIt(input, "shiny gold"))

  console.log(getAllTheBagsInsideCertainColoredBag(input, "shiny gold"))
})()

