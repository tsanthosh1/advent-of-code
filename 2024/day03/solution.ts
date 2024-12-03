import {readInput} from "../lib/input.ts";

export function solvePart1(data: string[]): number {
    return data.reduce((sum, line) => {
        const regExp = /mul\((\d+),(\d+)\)/g;
        let match;
        while ((match = regExp.exec(line)) !== null) {
            sum += Number(match[1]) * Number(match[2]);
        }
        return sum;
    }, 0);
}

export function solvePart2(data: string[]): number {
    const input = data.join("");
    const regExp = /mul\((\d+),(\d+)\)|don't\(\)|do\(\)/g;
    let match, dont = false, result = 0;

    while ((match = regExp.exec(input)) !== null) {
        if (match[0] === "don't()") {
            dont = true;
        } else if (match[0] === "do()") {
            dont = false;
        } else if (match[0].startsWith("mul") && !dont) {
            const num1 = Number(match[1]);
            const num2 = Number(match[2]);
            result += num1 * num2;
        }
    }

    return result;
}

readInput("day03").then((input) => {
    console.log("Part 1 Solution:", solvePart1(input));
    console.log("Part 2 Solution:", solvePart2(input));
});
