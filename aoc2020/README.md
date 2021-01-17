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
