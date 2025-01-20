# Day 19: Languages are Huge
This problem is cool in that it was the first of the 2015 problems that required significant changes between the first part and the second part. I put this readme here because I'm still not sure *why* I get the right answer.

## What I Tried
The first thing I tried was to do a BFS of all the generated strings starting from `e`. That works for small, short strings with a small number of rules. It blows up fast for longer strings. Even working in reverse (all the rules either keep the same length or add characters, so working in reverse only shrinks the string), I only get through 4 generations before dying due to being out of memory (with 32 gigs of RAM). The string is over 400 characters long, so we're nowhere near the right number of generations.

## What Sort of Worked
Instead, I wrote recursive DFS that uses a mutable `String` so that the rules are applied in-place. This makes the memory footpring rediculously tiny. There's a little bit of logic so that once a solution is found at a certain depth, future recursive calls will terminate at that depth, but I'm not sure how much that helps. Even with the in-place modification, the early termination, and a release build, I haven't actually seen the DFS terminate yet.

## Ideas
Even before the early termination code was in there, I got the same answer a bunch of times. That makes me think that ther's only one set of transformations that actually works, and the order is all that I'm varying. In that case, memoization would help enormously. The need for memoization in this case comes from the fact that applying the rules in different orders will generate the same string, so there's probaly a ton of gain to be had there.

# Cool Rust facts
Normally (in C) passing a pointer to a string to a series of calls, each of which modify it, would have resulted in at least segfault while I was testing it. I wasn't sure I could write Rust code that could actually do that, but it was surprisingly easy. It turns out that not only does Rust give you the tools to do that safely, it takes a ton of the load off by providing `String::replace_range()` which avoids a lot of off-by-one opportunities, and will panic if you exceed the capacity. Cool stuff!