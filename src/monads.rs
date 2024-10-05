// Counter monad
type Counter<T> = (T, u32);

fn to_upper_case(a: String) -> Counter<String> {
    (a.to_uppercase(), 1)
}

fn to_words(a: String) -> Counter<Vec<String>> {
    ((&a).split(' ').map(|s| s.to_string()).collect(), 1)
}

fn id<A>(x:A) -> A {
    x
}

fn return_counter<A>(a:A) -> Counter<A> {
    (a, 0)
}

// (a -> Counter b) -> (b -> Counter c) -> (a -> Counter c)
fn fish_counter<A, B, C>(m1: impl Fn(A) -> Counter<B>, m2: impl Fn(B) -> Counter<C>) -> impl Fn(A) -> Counter<C> {
    move |a| {
        let (b, n1) = m1(a); // apply m1
        let (c, n2) = m2(b); // apply m2
        (c, n1 + n2) // concat, repack
    }
}

// (a -> b) -> (Counter a -> Counter b)
// fmap "lifts" functions. Functors map morphisms.
fn fmap_counter<A, B>(f: fn(A) -> B) -> impl Fn(Counter<A>) -> Counter<B> {
    fish_counter(id, move |x| return_counter(f(x)))
}

// How m1=id, m2=|a| return_counter(f(a)) as args to fish_counter make fmap_counter:
// (Counter a -> Counter a) -> (a -> Counter(f(a)) -> (Counter a -> Counter b)

// Counter a -> (a -> Counter b) -> Counter b
fn bind_counter<A, B>(counter: Counter<A>, f: impl Fn(A) -> Counter<B>) -> Counter<B> {
    let (a, n1) = counter; // unpack
    let (b, n2) = f(a); // apply f
    (b, n1 + n2)    // concat, repack
}

// Counter (Counter a) -> Counter a
// flatten
fn join_counter<A>(counter: Counter<Counter<A>>) -> Counter<A> {
    let ((a, n1), n2) = counter; // unpack
    (a, n1 + n2) // concat, repack
}


// test
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fish_counter() {
        let m = fish_counter(to_upper_case, to_words);
        let (x, c) = m("hello world".to_string());
        assert_eq!(x, vec!["HELLO".to_string(), "WORLD".to_string()]);
        assert_eq!(c, 2);
    }

    #[test]
    fn test_fmap_counter() {
        let m = fmap_counter(|x| 10*x);
        let c1: Counter<_> = (5, 8);
        let c2 = m(c1);
        assert_eq!(c2, (50, 8));
    }

    #[test]
    fn test_bind_counter() {
        let c1: Counter<_> = (5, 0);
        let c2 = bind_counter(c1, |x| (10*x, 1));
        assert_eq!(c2, (50, 1));
    }
}
