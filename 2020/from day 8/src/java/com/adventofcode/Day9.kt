package com.adventofcode

import java.math.BigInteger
import java.nio.file.Files
import java.nio.file.Path

fun parseInputFromFile(): List<BigInteger> {
    val input = Files.readString(Path.of("./src/resources/data9"))
    return input.lines().filter { x -> x != "" }.map { x -> BigInteger(x) }
}

fun findIncorrectValueFromXmasCode(sequence: List<BigInteger>, preambleLength: Int): BigInteger {
    for (i in preambleLength until sequence.size) {
        val subList = sequence.subList(i - preambleLength, i);
        val numbers = findTwoNumbersSumming(subList, sequence[i]);
        if (numbers != null) continue
        else {
            println(subList)
            return sequence[i]
        }
    }
    throw Exception("No incorrect value found!")
}

fun findTwoNumbersSumming(list: List<BigInteger>, requiredSum: BigInteger): Pair<BigInteger, BigInteger>? {

    val number = list.find { x ->
        list.contains(requiredSum - x) &&
                x != requiredSum.divide(BigInteger.valueOf(2)) } ?: return null

    return Pair(number, requiredSum - number)
}

fun findContiguousSumSetWithSum(list: List<BigInteger>, requiredSum: BigInteger): Pair<Int, Int> {
    for (i in list.indices) {
        var sum = BigInteger.ZERO;
        for (j in i until list.size) {
            when ((list[j] + sum).compareTo(requiredSum)) {
                0 -> return Pair(i, j)
                -1 -> {
                    sum += list[j]
                    continue
                }
                else -> break
            }
        }
    }
    throw java.lang.Exception("not found")
}



fun getSumOfMinAndMax(list: List<BigInteger>): BigInteger? {
    return list.maxOrNull()?.let { list.minOrNull()?.plus(it) }
}


fun main() {

    val input = parseInputFromFile();

    val incorrectNumber = findIncorrectValueFromXmasCode(input, 25)
    println(incorrectNumber);

    val set = findContiguousSumSetWithSum(input, incorrectNumber);

    val sumOfMinAndMax = getSumOfMinAndMax(input.subList(set.first, set.second + 1))

    print(sumOfMinAndMax)

}
