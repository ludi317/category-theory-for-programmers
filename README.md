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
