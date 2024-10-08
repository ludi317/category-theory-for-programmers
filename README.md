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
## Section 3.6

3. Considering that Bool is a set of two values True and False, show that it forms two (set-theoretical) monoids with respect to, respectively, operator && (AND) and || (OR).
```text
- AND:
    - object: Bool Type = {True, False}
    - identity: True && a = a, a && True = a
    - associativity: (a && b) && c = a && (b && c)
- OR:
    - object: Bool Type = {True, False}
    - identity: False || a = a, a || False = a
    - associativity: (a || b) || c = a || (b || c)
```
4. Represent the Bool monoid with the AND operator as a category: List the morphisms and their rules of composition.
```text
- object: Bool Type = {True, False}
- morphisms: && True, && False
    - id = && True
    - associativity: (a && b) && c = a && (b && c)
- composition: 
    - && True && True = && True
    - && True && False = && False
    - && False && True = && False
    - && False && False = && False
```
5. Represent addition modulo 3 as a monoid category.
```text
- object: Z/3Z = {0, 1, 2}
- morphisms: + 0, + 1, + 2
    - id = + 0
    - associativity: (a + b) + c = a + (b + c)
- composition:
    - + 0 + 0 = + 0
    - + 0 + 1 = + 1
    - + 0 + 2 = + 2
    - + 1 + 0 = + 1
    - + 1 + 1 = + 2
    - + 1 + 2 = + 0
    - + 2 + 0 = + 2
    - + 2 + 1 = + 0
    - + 2 + 2 = + 1
```

## Section 4.4

1. Construct the Kleisli category for partial functions (define composition and identity).
```rust
#[derive(PartialEq,Debug)]
enum Optional<T> {
   Valid(T),
   NotValid,
}

fn identity<T>(x: T) -> Optional<T> {
   Valid(x)
}

fn compose<T, U, V>(
   f1: impl Fn(T) -> Optional<U>,
   f2: impl Fn(U) -> Optional<V>,
) -> impl Fn(T) -> Optional<V> {
   move |x: T| match f1(x) {
      Valid(f1out) => f2(f1out),
      NotValid => NotValid,
   }
}

fn safe_reciprocal(x: f64) -> Optional<f64> {
   if x == 0.0 {
      NotValid
   } else {
      Valid(1.0 / x)
   }
}

fn safe_root(x: f64) -> Optional<f64> {
   if x < 0.0 {
      NotValid
   } else {
      Valid(x.sqrt())
   }
}

#[test]
fn test_compose() {
   let safe_root_reciprocal = compose(safe_root, safe_reciprocal);
   assert_eq!(safe_root_reciprocal(4.0), Valid(0.5));
   assert_eq!(safe_root_reciprocal(-1.0), NotValid);
   assert_eq!(safe_root_reciprocal(0.0), NotValid);
   let safe_root_identity = compose(safe_root, identity);
   assert_eq!(safe_root_identity(4.0), Valid(2.0));
   assert_eq!(safe_root_identity(-1.0), NotValid);
   let identity_safe_root = compose(identity, safe_root);
   assert_eq!(identity_safe_root(4.0), Valid(2.0));
   assert_eq!(identity_safe_root(-1.0), NotValid);
}

```
## Section 5.8
4. Implement the equivalent of Haskell Either as a generic type in your favorite language (other than Haskell).
```rust
#[derive(PartialEq,Debug)]
enum Either<L,R> {
    Left(L),
    Right(R)
}
```
5. Show that Either is a “better” coproduct than int equipped with two injections i and j:
```text
There exists a function m from Either to int that factorizes i and j, but there is no m' function from int to Either that factorizes i and j. 
That's because i(0) = 0 and j(true) = 0. You could not tell if 0 came from i or j, and whether it should be a Left or Right.
```
```rust

fn i(n: i32) -> i32 {
    n
}

fn j(b: bool) -> i32 {
    if b { 0 } else { 1 }
}

// factorizer :: (a -> c) -> (b -> c) -> Either a b -> c
// returns the function m that factorizes i and j
fn factorizer<A,B,C>(i: fn(n: A) -> C, j: fn(b: B) -> C) -> impl Fn(Either<A, B>) -> C {
    move |x: Either<A, B>| {
        match x {
            Either::Left(a) => i(a),
            Either::Right(b) => j(b),
        }
    }
}

 #[test]
 fn test_factorizer() {
     let m = factorizer(i, j);
     let a: Either<i32, bool> = Either::Left(2);
     assert_eq!(m(a), 2);
     let b: Either<i32, bool> = Either::Right(false);
     assert_eq!(m(b), 1);
 }
```
## Section 6.5
1. Show the isomorphism between Maybe a and Either () a.
```rust
fn either_to_maybe<A>(x: Either<(), A>) -> Option<A> {
   match x {
      Either::Left(()) => None,
      Either::Right(a) => Some(a),
   }
}

fn maybe_to_either<A>(x: Option<A>) -> Either<(), A> {
   match x {
      None => Either::Left(()),
      Some(a) => Either::Right(a),
   }
}

    #[test]
    fn test_either_to_maybe() {
        let a: Either<i32, ()> = Either::Left(1);
        assert_eq!(maybe_to_either(either_to_maybe(a)), a);
        let b: Either<i32, ()> = Either::Right(());
        assert_eq!(either_to_maybe(b), None);
    }
```
## Sec