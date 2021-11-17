# Derangement

Library used to generate a random derangement and map values, calculate the inverse and get the underlying map

# What is a derangement?

A derangement is a permutation with no fixed points

# What is a permutation?

We can think of a permutation just a rearrangement of a set of values. So if we have the list [0, 1, 2] then a permutation of this would be [0, 2, 1].

To get from the former to the latter, we note that 0 maps to 0, 1 maps to 2 and 2  maps to 1. We can write this as 

(0 1 2)
(0 2 1)

Where the number on top maps to the number underneath. We can also write this in cyclic form, where each number maps to the next in the cycle, and cycles are separated by brackets:

(0)(1 2)

Also note that the order if a cycle is the number of elements in it (so the order of (0) is 1, and the order of (1 2) is 2)

If an element maps to itself, it is a fixed point. It also is within a cycle of order 1 (like 0 in the above example)

# How do we generate a random derangement?

The most important aspect of the derangement algorithm is noting that a derangement is a permutation group where all the cycles have order greater than 2. This means we can use this fact to partition a set of numbers into cycles with an order greater than 2.

So here we define an algorithm to randomly generate derangement:

1. Generate a list of numbers from 0 to size-1
2. Randomly shuffle this list
3. Generate a random number and use this as the cycle size.
4. Repeat step 3 until there are no elements left to partition, or there are 2 elements left 

Each time we partition the list, is esential that we do not allow any cycles of order 1, and therefore prevent fixed points. So if we  have `n` elements left to partition then the next partition can be in the range ` [2, n-1) ∩ { n }`.

We cannot allow a partition of `n-1` as this would leave one element which would become a fixed point.