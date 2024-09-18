# Solutions to Challenges in Category Theory for Programmers

## Section 1.4

1. Implement, as best as you can, the identity function in your favorite language (or the second favorite, if your favorite language happens to be
   Haskell).

```rust
fn id<T>(x: T) -> T {
    x
}
```

2. Implement the composition function in your favorite language. It takes
   two functions as arguments and returns a function that is their composition.
```rust
fn compose<A, B, C>(f: fn(A) -> B, g: fn(B) -> C) -> impl Fn(A) -> C {
   move |x| g(f(x))
}
```

3. Write a program that tries to test that your composition function respects
   identity.
```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compose() {
        let f = |x: i32| 3*x + 1;
        for x in 0..100 {
            assert_eq!(compose(f, id)(x), f(x));
            assert_eq!(compose(id, f)(x), f(x));
        }
    }
}
```
4. Is the world-wide web a category in any sense? Are links morphisms?
```text
- objects are web pages
- morphisms/arrows are hyperlinks
    - composition of morphisms: linking from A -> B and B -> C implies a link from A -> C
    - identity morphisms are self-links
```
5. Is Facebook a category, with people as objects and friendships as morphisms?
```text
- friendship is not composable: if A is friends with B and B is friends with C, it does not imply that A is friends with C
```
6. When is a directed graph a category?
```text
- identity: every node has a self-loop
- composable: for every pair of nodes A and B, if there is a path from A to B and a path from B to C, there is a path from A to C
```
## Section 2.7
1. Define a higher-order function (or a function object) memoize in your favorite language. This function takes a pure function f as an argument and
   returns a function that behaves almost exactly like f, except that it only
   calls the original function once for every argument, stores the result internally, and subsequently returns this stored result every time it’s called
   with the same argument. You can tell the memoized function from the
   original by watching its performance. For instance, try to memoize a function that takes a long time to evaluate. You’ll have to wait for the result
   the first time you call it, but on subsequent calls, with the same argument, you should get the result immediately.
```rust
use std::collections::HashMap;
fn memoize<A, B, F>(mut f: F) -> impl FnMut(A) -> B
where
        A: Eq + Hash + Clone,
        B: Clone,
        F: FnMut(A) -> B,
{
   let mut cache: HashMap<A, B> = HashMap::new();
   move |x: A| {
      if cache.contains_key(&x) {
         cache[&x].clone()
      } else {
         let y = f(x.clone());
         cache.insert(x, y.clone());
         y
      }
   }
}
```
2. Try to memoize a function from your standard library that you normally use to produce random numbers. Does it work?
```rust
use rand::Rng;
#[test]
fn test_memoize() {
   let mut rng = rand::thread_rng();
   let mut memoized_rand = memoize(|_| rng.gen::<u32>());
   for _ in 0..10 {
      println!("{}", memoized_rand(()));
   }
}
```
No, it does not work. The random number generator is not pure.

3. Most random number generators can be initialized with a seed. Implement a function that takes a seed, calls the random number generator with that seed, and returns the result. Memoize that function. Does it work?
```rust
#[test]
fn test_memoize_seed() {
   let mut memoized_rand = memoize(|seed: u32| {
      let mut rng = rand::rngs::StdRng::seed_from_u64(seed as u64);
      rng.gen::<u32>()
   });
   for _ in 0..10 {
      println!("{}", memoized_rand(42));
   }
}
```
Yes, it works. The random number generator is now pure. 

4. Which of these C++ functions are pure? Try to memoize them and observe what happens when you call them multiple times: memoized and not.
```text
(a) - pure; (b), (c), (d) - not pure
```
5. How many different functions are there from Bool to Bool? Can you implement them all?
```rust
fn id(x: bool) -> bool {
    x
}

fn not(x: bool) -> bool {
    !x
}

fn always(_: bool) -> bool {
    true
}

fn never(_: bool) -> bool {
    false
}
```

