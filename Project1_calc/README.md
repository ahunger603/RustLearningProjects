#What I did

The homework was straight forward. The goal I had for myself in this homework was to use as many Rust tools as possible to begin to understand what the language has to offer. In the this project, I tried using Closures, Results, match statements, Vectors, Boxes, and other general language basics.

#How it went

Overall the homework went really well. I have enjoyed everything Rust has had to offer so far. The syntax for Closures, and types in general has been something to get used to though. Very excited to continue working within Rust.

#How I tested it

Testing was extremely straight forward. I kept all of the primary "testable" functionality within the `calc` function. The calc function returns a Result. The first set of tests check that invalid inputs produce an `Err`. The remaining tests check each of the use cases: sum, product, gcd and lcm. For each, checking if correct inputs produce an `Ok` and match known results.