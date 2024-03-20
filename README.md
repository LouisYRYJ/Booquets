# Booquets


Booquets is a **Boo**lean **que**ry **t**ree **s**earcher.  This is a little project I wrote to familiarise myself with LL(1) parsing, Rust, and also TDD.


## About 

Search documents for boolean queries, i.e. queries of the form 
> (color OR colour) AND ((flowers OR starry night) AND Dutch painter)


### Usage

To use it, run 

```
cargo run -- "query" file path
```

**Right now only a single text file can be searched, but pdf and multiple files will be possible as well soon.**

The query should be specified as follows:
- "\+" represents OR, "\*" represents AND
- Terms encapsulated by ( ) are parsed first, and \* takes precedence over \+
- Use the environment variables IGNORE_CASE to make the search case insensitive and DISPLAY_TREE to also see the parsed tree of the query

The query in the About section then can look like (with the tree displayed) 

```
DISPLAY_TREE=1 cargo run -- "(color + colour) *((flowers + starry night) * Dutch painter) " file path
```

### Tests

Run tests with 
```
cargo test
```
Some tests output trees, to be able to view the trees run  
```
cargo test -- --nocapture --test-threads=1
```
### What it is doing

1. First the query is parsed into a tree using an LL(1)-parser. In the above example it looks like this:
```
*
├─ +
│  ├─ color
│  └─ colour
└─ *
   ├─ +
   │  ├─ flowers
   │  └─ starry night
   └─ Dutch painter
```


2. Using a breadth-first search, the lowest depth leaf determined (starting from the left). In this case it would be "color". 

Note: There are better policies to determine the next query, for example in 
```
├─ +
│  ├─ +
│  │  ├─ A
│  │  └─ B
│  └─ *
│     ├─ A
│     └─ C
└─ *
   ├─ +
   │  ├─ A
   │  └─ C
   └─ B
```
searching for A is better than B, although B is the closest leaf. To be really sure to find the optimal next leaf, it seems like you would have to compute the expected tree size for each leaf though.

2. The document gets searched for this query.


3. Based on whether it was found or not, the tree is updated. Let's say "color" was found. Then the tree is updated once
```
*
├─ colour 
└─ *
   ├─ +
   │  ├─ flowers
   │  └─ starry night
   └─ Dutch painter
```
In this example we are done, but more generally we have to repeatedly update the tree until either "color" does not appear in the tree anymore or "color" is the only leaf left.

4. As described above, we check if there is only one leaf left. In that case we are done. Otherwise we go back to 2. and repeat.
