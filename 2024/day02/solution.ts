import {readInput} from "../lib/input.ts";

export function solvePart1(data: string[]): number {
    return data.reduce((count, line) => {
        const numbers = parseNumbers(line);
        return count + (isValidReport(numbers) ? 1 : 0);
    }, 0);
}

export function solvePart2(data: string[]): number {
    return data.reduce((count, line) => {
        const numbers = parseNumbers(line);
        return count + (isValidReport(numbers) || retryWithDampener(numbers) ? 1 : 0);
    }, 0);
}

function parseNumbers(line: string): number[] {
    return line.split(" ").map(Number);
}

function retryWithDampener(numbers: number[]): boolean {
    return numbers.some((_v, i) => 
        isValidReport(numbers.filter((_, j) => j !== i)));
}

function isValidReport(numbers: number[]): boolean {
    let direction: "increasing" | "decreasing" | undefined;

    for (let i = 1; i < numbers.length; i++) {
        const difference = numbers[i - 1] - numbers[i];
        const currentDirection = difference > 0 ? "increasing" : "decreasing";

        if (Math.abs(difference) > 3 || difference === 0) {
            return false;
        }

        if (!direction) {
            direction = currentDirection;
        } else if (direction !== currentDirection) {
            return false;
        }
    }

    return true;
}


const input = await readInput("day02");
console.log("Part 1 Solution:", solvePart1(input));
console.log("Part 2 Solution:", solvePart2(input));
