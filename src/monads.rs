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

// a -> m a
fn return_counter<A>(a:A) -> Counter<A> {
    (a, 0)
}

// (a -> m b) -> (b -> m c) -> (a -> m c)
fn fish_counter<A, B, C>(m1: impl Fn(A) -> Counter<B>, m2: impl Fn(B) -> Counter<C>) -> impl Fn(A) -> Counter<C> {
    move |a: A| {
        let (b, n1) = m1(a); // apply m1
        let (c, n2) = m2(b); // apply m2
        (c, n1 + n2) // concat, repack
    }
}

// (a -> b) -> (m a -> m b)
fn fmap_counter<A, B>(f: impl Fn(A) -> B) -> impl Fn(Counter<A>) -> Counter<B> {
    fish_counter(id, move |x: A| return_counter(f(x)))
}

// (a -> b) -> (m a -> m b)
fn fmap_counter2<A, B>(f: impl Fn(A) -> B) -> impl Fn(Counter<A>) -> Counter<B> {
    move |(a, n): Counter<A> | (f(a), n)
}

// (a -> b) -> (m a -> m b)
fn fmap_counter3<A, B>(f: impl Fn(A) -> B) -> impl Fn(Counter<A>) -> Counter<B> {
    move |c: Counter<A>| {
        bind_counter(c, |x| return_counter(f(x)))
    }
}
// m a -> (a -> m b) -> m b
fn bind_counter<A, B>(c: Counter<A>, f: impl Fn(A) -> Counter<B>) -> Counter<B> {
    let (a, n1) = c; // unpack
    let (b, n2) = f(a); // apply f
    (b, n1 + n2)    // concat, repack
}

// m (m a) -> m a
// flatten
fn join_counter<A>(counter: Counter<Counter<A>>) -> Counter<A> {
    let ((a, n1), n2) = counter; // unpack
    (a, n1 + n2) // concat, repack
}

// m a -> (a -> m b) -> m b
// bind in terms of join and fmap
fn bind_counter2<A, B>(counter: Counter<A>, f: impl Fn(A) -> Counter<B>) -> Counter<B> {
    join_counter(fmap_counter(f)(counter))
}

/*
Monoid laws:
1. identity element
2. associative binary operation

Functor laws. A type constructor and its fmap function are a functor if they obey the following laws:
- A functor applies a function to a wrapped value
- fmap "lifts" functions. Functors map morphisms.
fmap :: (a -> b) -> m a -> m b
1. fmap id = id                     -- identity
2. fmap (f . g) = fmap f . fmap g   -- composition

Monad laws (ie standard composition laws for Kleisli category):
- A way of composing embellished functions
- Requires a type constructor with return and bind functions
- (a -> m b) is a Kleisli arrow
(>=>) :: (a -> m b) -> (b -> m c) -> a -> m c
1. (f >=> g) >=> h = f >=> (g >=> h) -- associativity
2. return >=> f = f                  -- left unit
3. f >=> return = f                  -- right unit

Monads are functors. Bind and return give fmap.
Monads are a high-level monoid. A monad is a monoid in the category of endofunctors.
 */

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
        let c1: Counter<_> = (5, 2);
        let c2 = bind_counter(c1, |x| (10*x, 1));
        assert_eq!(c2, (50, 3));
    }

    #[test]
    fn test_bind_counter2() {
        let c1: Counter<_> = (5, 2);
        let c2 = bind_counter2(c1, |x| (10*x, 1));
        assert_eq!(c2, (50, 3));
    }

    #[test]
    fn test_join_counter() {
        let c1: Counter<_> = ((6, 2), 3);
        let c2 = join_counter(c1);
        assert_eq!(c2, (6, 5));
    }

}
