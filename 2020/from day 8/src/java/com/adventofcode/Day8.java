package com.adventofcode;

import lombok.SneakyThrows;

import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static java.util.stream.Collectors.toList;

public class Day8 {

    public static List<BootCodeInstruction> parseInput(String input) {
        return input.lines().map(BootCodeInstruction::from).collect(toList());
    }


    @SneakyThrows
    public static void main(String[] args) {
        String input = Files.readString(Path.of("./src/resources/data8"));

        var bootCode = new BootCode(parseInput(input));

        bootCode.execute();

        System.out.println(bootCode.getAccumulator());

        bootCode.reset();
        new BootCodeDebugger().fixInfiniteLoop(bootCode);

        System.out.println(bootCode.getAccumulator());

    }
}
