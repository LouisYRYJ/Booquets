# Booquets


Booquets is a **Boo**lean **que**ry **t**ree **s**earcher written in Rust. 

## About 

Search documents for boolean queries, i.e. queries of the form 
> (color OR colour) AND ((flowers OR starry night) AND (Dutch painter))

(Warning: This is just a mini project, so not very efficient)

### Usage

To use it, run 

```
cargo run -- "query" file path
```
where the query should be specified as follows:
- "\+" represents OR, "\*" represents AND
- Terms encapsulated ( ) are parsed first, and \* takes precedence over \+
- Use the environment variables IGNORE_CASE to make the search case insensitive and DISPLAY_TREE to also see the parsed tree of the query

The query in the About section then can look like (with the tree displayed) 

```
DISPLAY_TREE=1 cargo run -- "(color + colour) *((flowers + starry night) * Dutch painter) " file path
```

### What it is doing

First the query is parsed into a tree. In the above example it looks like this:

![](<Binary Search Tree Example.png>)

There are multiple ways to parse the query, I chose to stick with the one used in this [arithmetic expression parser](https://github.com/gnebehay/parser). Then:

1. The closest leaf (which is equivalent to being simply a query i.e. not an AND or OR operator) is determined (in this case it would be "color"). 
2. The document gets searched for this query
3. Based on whether it was found or not, the tree is updated
4. Check if we are done and if not we go back to 1.