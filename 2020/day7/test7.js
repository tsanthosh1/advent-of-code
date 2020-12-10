
import assert from 'assert'
import {
  getAllTheBagsInsideCertainColoredBag,
  getBagsColorThatCanContainCertainColoredBagInIt,
  readInputFromData
} from "./seven";

const testData1 = `
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
`

describe('luggage color rules', () => {
  it('should get count of all colors that can contain specific color', () => {

    let input = readInputFromData(testData1.split('\n').filter(x => Boolean(x)));

    assert.strictEqual(getBagsColorThatCanContainCertainColoredBagInIt(input, "shiny gold"), 4);
  });

  it('should return number of bags within specific colored bag', () => {

    let input = readInputFromData(testData1.split('\n').filter(x => Boolean(x)));

    assert.strictEqual(getAllTheBagsInsideCertainColoredBag(input, "shiny gold"), 32)
  });
});
