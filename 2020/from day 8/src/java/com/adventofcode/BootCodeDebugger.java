package com.adventofcode;

import java.util.Optional;
import java.util.function.Predicate;

import static com.adventofcode.BootCodeOperation.JUMP;
import static com.adventofcode.BootCodeOperation.NO_OPERATION;

public class BootCodeDebugger {


    public BootCode fixInfiniteLoop(BootCode bootCode) throws Exception {

        var instructions = bootCode.getInstructions();

        Predicate<BootCodeInstruction> filterNegativeJump = instruction ->
                instruction.getOperation().equals(JUMP) && instruction.getArgument() < 0;

        Predicate<BootCodeInstruction> filterPositiveJump = instruction ->
                instruction.getOperation().equals(JUMP) && instruction.getArgument() >= 0;

        Predicate<BootCodeInstruction> filterNoOperation = instruction ->
                instruction.getOperation().equals(NO_OPERATION);


        var result = tryDebug(bootCode, filterNegativeJump, NO_OPERATION, JUMP);
        if (result.isPresent()) return bootCode;


        result = tryDebug(bootCode, filterPositiveJump, NO_OPERATION, JUMP);
        if (result.isPresent()) return bootCode;

        result = tryDebug(bootCode, filterNoOperation, JUMP, NO_OPERATION);
        if (result.isPresent()) return bootCode;

        throw new Exception("Could not fix infinite loop");
    }

    private Optional<BootCodeInstruction> tryDebug(BootCode bootCode, Predicate<BootCodeInstruction> filterNegativeJump,
                                                   BootCodeOperation to, BootCodeOperation from) {

        var instructions = bootCode.getInstructions();
        return instructions.stream()
                .filter(filterNegativeJump)
                .filter(instruction -> {
                    instruction.setOperation(to);
                    bootCode.reset();
                    boolean state = bootCode.execute();
                    if (!state) instruction.setOperation(from);
                    return state;
                }).findFirst();
    }

}
