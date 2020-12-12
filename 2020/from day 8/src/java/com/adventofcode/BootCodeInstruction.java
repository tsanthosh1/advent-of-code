package com.adventofcode;

import lombok.Getter;
import lombok.Setter;

import static java.lang.Integer.parseInt;

@Getter
@Setter
public class BootCodeInstruction {

    private BootCodeOperation operation;
    int argument;

    public BootCodeInstruction(BootCodeOperation operation, int argument) {
        this.operation = operation;
        this.argument = argument;
    }


    public static BootCodeInstruction from(String instruction) {
        String[] split = instruction.split(" ");
        return new BootCodeInstruction(BootCodeOperation.from(split[0]), parseInt(split[1]));
    }


}

