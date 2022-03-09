pub trait MapReduce {
    type S;
    /// F is a function that takes a value of the iterator S and tranansforms it into Ts.
    /// G is a function that takes the results from F and transforms them into a single value.
    /// G should be a commutative assosiative binary operator
    fn map_reduce<T, F, G>(self, map_func: F, reduce_func: G) -> Option<T>
    where
        F: Fn(Self::S) -> T,
        G: Fn(T, T) -> T;
}

impl<I: Iterator> MapReduce for I {
    type S = I::Item;

    fn map_reduce<T, F, G>(self, map_func: F, reduce_func: G) -> Option<T>
    where
        F: Fn(Self::S) -> T,
        G: Fn(T, T) -> T,
    {
        let mut c = self.map(map_func);
        c.next().map(|s| c.fold(s, reduce_func))
    }
}
