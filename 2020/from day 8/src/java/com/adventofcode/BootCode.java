package com.adventofcode;

import lombok.*;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

import static com.adventofcode.BootCodeOperation.JUMP;
import static com.adventofcode.BootCodeOperation.NO_OPERATION;

@Getter
@Setter
@Builder
@NoArgsConstructor
@AllArgsConstructor
@RequiredArgsConstructor
public class BootCode {
    @NonNull
    @Singular
    private List<BootCodeInstruction> instructions;

    private int accumulator = 0;
    private int currentInstructionIndex = 0;
    private List<Integer> executedInstructions = new ArrayList<>();

    public boolean execute() {

        while(currentInstructionIndex < instructions.size()) {
            var instruction = instructions.get(currentInstructionIndex);
            if (executedInstructions.contains(currentInstructionIndex)) return false;
            executedInstructions.add(currentInstructionIndex);

            switch (instruction.getOperation()) {
                case ACCUMULATOR -> {
                    accumulator += instruction.getArgument();
                    currentInstructionIndex++;
                }
                case JUMP -> currentInstructionIndex += instruction.getArgument();
                case NO_OPERATION -> currentInstructionIndex++;
            }
        }

        return true;
    }


    public void reset() {
        accumulator = 0;
        currentInstructionIndex = 0;
        executedInstructions = new ArrayList<>();
    }
}

