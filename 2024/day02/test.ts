import {assertEquals} from "https://deno.land/std/assert/mod.ts"; // Updated import path
import {solvePart1, solvePart2} from "./solution.ts";

Deno.test("solvePart1 with mock data", () => {
    const mockData = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9"]
    assertEquals(solvePart1(mockData), 2);
});

Deno.test("solvePart1 with mock data", () => {
    const mockData = [
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9"]
    assertEquals(solvePart2(mockData), 4);
});


