import {assertEquals} from "https://deno.land/std/assert/mod.ts"; // Updated import path
import {solvePart1, solvePart2} from "./solution.ts";

Deno.test("solvePart1 with mock data", () => {
    const mockData = ["199", "200", "208", "210"];
    assertEquals(solvePart1(mockData), 817);
});

Deno.test("solvePart1 with mock data", () => {
    const mockData = ["199", "200", "208", "210"];
    assertEquals(solvePart2(mockData), 817);
});


