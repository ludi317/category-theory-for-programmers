
struct Reader<E, A> {
    run: Box<dyn Fn(E) -> A>,
}

impl <E: 'static + Copy, A: 'static> Reader<E, A> {
    // (e -> a) -> Reader(e -> a)
    fn new<F: 'static + Fn(E) -> A>(f: F) -> Reader<E, A> {
        Reader{run:Box::new(f)} // wrap
    }

    // (a -> b) -> Reader(e -> a) -> Reader(e -> b)
    fn fmap<B: 'static, F: 'static + Fn(A) -> B>(self, f: F) -> Reader<E, B> {
        Reader::new(move |env: E| { // Reader(e -> b)
            let a = (self.run)(env); // a
            f(a) // b
        })
    }

    // Reader(e -> a) -> e -> a
    fn run(self, env: E) -> A {
        (self.run)(env)
    }

    // Reader(e -> a) -> (a -> Reader(e -> b)) -> Reader(e -> b)
    fn bind<B: 'static, F: 'static + Fn(A) -> Reader<E, B>>(self, f: F) -> Reader<E, B> {
        Reader::new(move |env: E| { // Reader(e -> b)
            let a = (self.run)(env); // a
            let next_reader = f(a); // Reader(e -> b)
            (next_reader.run)(env) // b
        })
    }
}

fn pure<E: Copy + 'static, A: 'static + Copy>(a: A) -> Reader<E, A> {
    Reader::new(move |_| a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmap() {
        let double_reader = Reader::new(|env: i32| env * 2);
        let double_then_add_one = double_reader.fmap(|x| x + 1);
        assert_eq!(double_then_add_one.run(10), 21);
    }

    #[test]
    fn test_bind() {
        let double_reader = Reader::new(|env: i32| env * 2);
        let bound_reader = double_reader.bind(|x| Reader::new(move |env: i32| env + x));
        assert_eq!(bound_reader.run(10), 30);
    }

    #[test]
    fn test_pure() {
        let pure_reader = pure::<i32, i32>(42);
        assert_eq!(pure_reader.run(0), 42);
    }

    #[test]
    fn test_fmap_with_pure() {
        let pure_reader = pure::<i32, i32>(42);
        let add_one_reader = pure_reader.fmap(|x| x + 1);
        assert_eq!(add_one_reader.run(0), 43);
    }

    #[test]
    fn test_bind_with_pure() {
        let pure_reader = pure::<i32, i32>(42);
        let bound_reader = pure_reader.bind(|_| pure(100));
        assert_eq!(bound_reader.run(0), 100);
    }
}

