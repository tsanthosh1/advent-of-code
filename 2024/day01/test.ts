import { assertEquals } from "https://deno.land/std/assert/mod.ts"; // Updated import path
import {solvePart1, solvePart2} from "./solution.ts";

Deno.test("solvePart1 with mock data", () => {
    const mockData = ["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
    assertEquals(solvePart1(mockData), 11);
    assertEquals(solvePart2(mockData), 31);
});
