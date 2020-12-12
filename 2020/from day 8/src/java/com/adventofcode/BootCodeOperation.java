package com.adventofcode;

import lombok.Getter;

import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

@Getter
public enum BootCodeOperation {
    ACCUMULATOR("acc"), JUMP("jmp"), NO_OPERATION("nop");

    private final String op;

    BootCodeOperation(String op) {
        this.op = op;
    }

    private static final Map<String,BootCodeOperation> map;

    static {
        map = new HashMap<>();
        Arrays.stream(BootCodeOperation.values()).forEach(x -> map.put(x.op, x));
    }
    public static BootCodeOperation from(String op) {
        return map.get(op);
    }
}
