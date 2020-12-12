package com.adventofcode;

import lombok.SneakyThrows;
import lombok.val;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;

import static com.adventofcode.Day8.parseInput;
import static org.junit.jupiter.api.Assertions.*;

class BootCodeDebuggerTest {

    @SneakyThrows
    @Test
    void shouldFixInfiniteLoop() {
        val testData = """
                nop +0
                acc +1
                jmp +4
                acc +3
                jmp -3
                acc -99
                acc +1
                jmp -4
                acc +6
                """;

        var bootCode = BootCode.builder()
                .executedInstructions(new ArrayList<>())
                .instructions(parseInput(testData)).build();

        new BootCodeDebugger().fixInfiniteLoop(bootCode);

        assertEquals(8, bootCode.getAccumulator());
    }

}
