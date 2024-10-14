trait Bifunctor<A, B> {
    type Output<A2, B2>;

    fn bimap<A2, B2, F, G>(self, f: F, g: G) -> Self::Output<A2, B2>
    where
        F: FnOnce(A) -> A2,
        G: FnOnce(B) -> B2;
}

impl<A, B> Bifunctor<A, B> for (A, B) {
    type Output<A2, B2> = (A2, B2);

    fn bimap<A2, B2, F, G>(self, f: F, g: G) -> Self::Output<A2, B2>
    where
        F: FnOnce(A) -> A2,
        G: FnOnce(B) -> B2,
    {
        let (a, b) = self;
        (f(a), g(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bifunctor() {
        let pair = (1, "Hello".to_string());

        // Apply bimap to the pair
        let new_pair = pair.bimap(|x| x + 1, |s| s + " World");

        assert_eq!(new_pair, (2, "Hello World".to_string()));
    }
}

