import {assertEquals} from "https://deno.land/std/assert/mod.ts"; // Updated import path
import {solvePart1, solvePart2} from "./solution.ts";

Deno.test("solvePart1 with mock data", () => {
    const mockData = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))", "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];
    assertEquals(solvePart1(mockData), 161 * 2);
});

Deno.test("solvePart2 with mock data", () => {
    const mockData = [
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ];
    assertEquals(solvePart2(mockData), 48);
});


