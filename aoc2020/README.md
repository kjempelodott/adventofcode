![alt text](https://rustacean.net/assets/rustacean-flat-happy.svg)

## Advent of code 2020
_Summary of problems and notes on solution and/or implementation_

#### Day 1
###### Problem
From a list of numbers _N_, find exactly _y_ numbers that sum to _X_
###### Solution
Naive solution with _y_ nested iterators. Will fail if any _n_ = _X_ / _y_

#### Day 2
###### Problem
Count letters
###### Solution
Trivial. The [parse_display](https://docs.rs/parse-display/) crate came in handy for parsing input.

#### Day 3
###### Problem
Traverse a binary 2D-grid that wraps around the _x_-axis. The same move is done on each iteration.
###### Solution
Store the indices of occupied (trees) points in a double vector

#### Day 4
###### Problem
Validate input based on a set of rules. Input is a mix of numbers and strings
###### Solution
Trivial. The [parse_display](https://docs.rs/parse-display/) crate came in handy for parsing input.

#### Day 5
###### Problem
Binary partitioning with 7 and 3 bits
###### Solution
Convert each input string to a 10-bit number. The 7 leading bits (seat row) span a space of 128 values, the 3 last (seat column) span a space of 8 values. The seat ID is given by multiplying row number with 8 and adding the column number. When converting the input to a 10-bit number, the row number bits are automatically left-shifted by 3, the same as mulitplying with 8.

#### Day 6
###### Problem
Union and intersection of chars
###### Solution
Trivial when using a hashset. The [Iterator::fold_first ](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold_first) function uses the first element in the iterator as the initial value, which is neat when e.g. intersecting an array of hashsets.

#### Day 7 – The Shiny Gold :sparkling_heart: :yellow_heart:
###### Problem
Directed acyclic graph (DAG)
###### Solution
This is a graph that can be topologically ordered, there are noe loops. Part 1 asks for all possible routes leading to a particular node (The Shiny Gold). This is solved with BFS sort. Part 2 asks for all nodes reachable from a particular starting node (The Shiny Gold). This is solved with DFS sort.

#### Day 8
###### Problem
Virtual Machine sort of problem. Part 2 is about fuzzing the input program.
###### Solution
Straightforward. The [parse_display](https://docs.rs/parse-display/) made conversion from input to "assembly" really smooth.

#### Day 9
###### Problem
Find _x_ and _y_ which sum equals _Z_ in a list of numbers. In part 2, find a contiguous set of numbers which sum is _Z_. Input is sorted
###### Solution
Part 1 is solved with nested iterators, similar to day 1. Part 2 is solved by scanning with a window of dynamic length that grows or moves to the left (lower numbers) until its sum matches _Z_.

#### Day10
###### Problem
Part 1 asks for the difference between all adjacent elements of an array. Part 2 asks for all possible sub-arrangements of the input numbers – combinatorics
###### Solution
Part 1 is trivial, and we learn that all differences are either 1 or 3. In all sub-arrangements, the numbers have to be sorted, and the first and last number must be present. The difference between any two adjacent numbers cannot be more than 3. This means that whenever there are _n_ = 3 or more contiguous numbers whose difference is 1, there are more than one way to make a sub-list that fulfills the requirements. Consider the two examples below, for _n_ = 3 and _n_ = 4.
```
[0 3 4 5 8] -> [0 3 4 5 8], [0 3 5 8]
[0 3 4 5 6 9] -> [0 3 4 5 6 9], [0 3 4 6 9], [0 3 5 6 9], [0 3 6 9]
```
In general, the number of sub-lists become <img src="https://render.githubusercontent.com/render/math?math=\sum_{i=1}^{n-2}i">
