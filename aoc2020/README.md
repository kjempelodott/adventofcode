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
