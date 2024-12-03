import {readInput} from "../lib/input.ts";

function getSortedLists(data: string[]) {
    const list1: number[] = [], list2: number[] = [];
    data.forEach(x => {
        const numbers = x.split("   ");
        list1.push(Number(numbers[0]))
        list2.push(Number(numbers[1]))
    });
    list1.sort((a, b) => a - b)
    list2.sort((a, b) => a - b)
    return {list1, list2};
}

export function solvePart1(data: string[]): number {
    const {list1, list2} = getSortedLists(data);

    let distance = 0
    for (let i = 0; i < list1.length; i++) {
        distance += Math.abs(list1[i] - list2[i]);
    }
    return distance;
}

export function solvePart2(data: string[]): number {
    const {list1, list2} = getSortedLists(data);

    const occurrence: Record<number, number> = {}
    let similarityScore = 0
    list2.forEach(x => occurrence[x] = occurrence[x] ? (occurrence[x] + 1) : 1)
    list1.forEach(x => {
        similarityScore += (x * (occurrence[x] || 0)) 
    })
    return similarityScore;
}

readInput("day01").then((input) => {
    console.log("Part 1 Solution:", solvePart1(input));
    console.log("Part 1 Solution:", solvePart2(input));
});
