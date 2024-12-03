import {readInput} from "../lib/input.ts";

export function solvePart1(data: string[]): number {
    return data.reduce((sum, val) => sum + parseInt(val, 10), 0);
}

export function solvePart2(data: string[]): number {
    return data.reduce((sum, val) => sum + parseInt(val, 10), 0);
}

readInput("day00").then((input) => {
    console.log("Part 1 Solution:", solvePart1(input));
    console.log("Part 1 Solution:", solvePart2(input));
});
