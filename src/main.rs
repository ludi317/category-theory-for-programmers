use rand::Rng;
use std::collections::HashMap;
use std::hash::Hash;
use crate::Optional::{NotValid, Valid};

fn main() {
    let m = factorizer(i, j);
    let a: Either<i32, bool> = Either::Left(2);
    let c = m(a);
    println!("{:?}", c );
    let b: Either<i32, bool> = Either::Right(false);
    let c = m(b);
    println!("{:?}", c )
}

#[derive(PartialEq,Debug)]
enum Either<L,R> {
    Left(L),
    Right(R)
}


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

// test
#[cfg(test)]
mod test {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn test_factorizer() {
        let m = factorizer(i, j);
        let a: Either<i32, bool> = Either::Left(2);
        assert_eq!(m(a), 2);
        let b: Either<i32, bool> = Either::Right(false);
        assert_eq!(m(b), 1);
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

    #[test]
    fn test_memoize() {
        let mut rng = rand::thread_rng();
        let mut memoized_rand = memoize(|_| rng.gen::<u32>());
        for _ in 0..10 {
            println!("{}", memoized_rand(()));
        }
    }

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
}
