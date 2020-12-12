package com.adventofcode;

import lombok.val;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import java.util.ArrayList;

import static com.adventofcode.BootCodeOperation.ACCUMULATOR;
import static com.adventofcode.Day8.parseInput;
import static org.junit.jupiter.api.Assertions.assertEquals;

class BootCodeTest {

    @Test
    void shouldExecuteAccumulatorInstruction() {
        var instruction = new BootCodeInstruction(ACCUMULATOR, 4);
        var bootCode = BootCode.builder()
                .executedInstructions(new ArrayList<>())
                .instruction(instruction).build();

        bootCode.execute();

        assertEquals(4, bootCode.getAccumulator());
    }

    @Test
    void shouldStopOnInfiniteLoopDetection() {
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

        bootCode.execute();

        assertEquals(5, bootCode.getAccumulator());
    }



}
