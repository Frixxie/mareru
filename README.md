# MAp REduce RUst

This is inspired by the MapReduce programming model.

## What is this? 

This is a crate that contains a trait that suggests a naive implementation of map reduce.

## How does it work? 

The trait is currently implemented for iterators, so every iterator can use this trait and its function. 

The function takes two functions, a map functions that transforms a set S to a result T. And a reduce function that reduces the result T(s) to one result T.
