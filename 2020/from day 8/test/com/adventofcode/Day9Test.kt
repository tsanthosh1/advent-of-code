package com.adventofcode

import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*
import java.math.BigInteger

internal class Day9Test {

    val testData = """
        35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576
    """.trimIndent()

    @Test
    fun shouldFindIncorrectValueFromXmasCode() {
        val sequence = testData.lines().map { x -> BigInteger(x) }.toList()
        val incorrectValue = findIncorrectValueFromXmasCode(sequence, 5)

        assertEquals(BigInteger.valueOf(127), incorrectValue)
    }


    @Test
    fun shouldFindContiguousSetWithSum() {

        val list = testData.lines().filter { x -> x != "" }.map { x -> BigInteger(x) }
        val set = findContiguousSumSetWithSum(list, BigInteger.valueOf(127))

        assertEquals(Pair(2, 5), set);
    }


    @Test
    internal fun shouldFindSumOfMinAndMax() {
        val list = testData.lines().filter { x -> x != "" }.map { x -> BigInteger(x) }
        val set = findContiguousSumSetWithSum(list, BigInteger.valueOf(127))

        val sumOfMinAndMax = getSumOfMinAndMax(list.subList(set.first, set.second + 1))
        assertEquals(BigInteger.valueOf(62), sumOfMinAndMax)
    }
}
