mod map_reduce;

pub use self::map_reduce::MapReduce;

#[cfg(test)]
mod map_reduce_tests {
    use super::*;
    use core::iter::repeat;
    #[test]
    fn str_to_int() {
        // This test takes a list of strings and converts them to integers and sums them.
        let words: Vec<String> = (0..1024).map(|i| format!("{}", i)).collect();
        let res = words
            .into_iter()
            .map_reduce(|w| i32::from_str_radix(&w, 10).unwrap_or(0), |a, b| a + b)
            .unwrap();
        // sum of 0..1024
        assert_eq!(res, (1 << 19) - (1 << 9));
    }

    #[test]
    fn numbers_sum() {
        // This test takes a list of numbers and sums them, here we can convert the numbers to
        // something else if we want.
        let nums = (0..(1 << 15)).collect::<Vec<_>>();
        let res = nums.iter().map_reduce(|n| *n, |a, b| a + b).unwrap();
        assert_eq!(res, (0..(1 << 15)).sum());
    }

    #[test]
    fn tuple_max() {
        // This test takes a list of tuples and returns the maximum of the second element.
        let data = vec![
            ("kake", 1),
            ("kake", 2),
            ("kake", 3),
            ("kake", 4),
            ("kake", 5),
            ("kake", 4),
            ("kake", 3),
            ("kake", 2),
            ("kake", 1),
        ];
        let res = data
            .into_iter()
            .map_reduce(
                |(_, t)| (1, t),
                |a, b| {
                    if a.1 > b.1 {
                        a
                    } else {
                        b
                    }
                },
            )
            .unwrap();
        assert_eq!(res, (1, 5));
    }

    #[test]
    fn tuple_min() {
        // This test takes a list of tuples and returns the minimum of the second element.
        let data = vec![
            ("kake", 1),
            ("kake", 2),
            ("kake", 3),
            ("kake", 4),
            ("kake", 5),
            ("kake", 4),
            ("kake", 3),
            ("kake", 2),
            ("kake", 1),
        ];
        let res = data
            .into_iter()
            .map_reduce(
                |(_, t)| t,
                |a, b| {
                    if a < b {
                        a
                    } else {
                        b
                    }
                },
            )
            .unwrap();
        assert_eq!(res, 1);
    }

    #[test]
    fn average() {
        // This test takes a list of 1s, doubles it calculates the average.
        let data = repeat(1).take(1 << 16).collect::<Vec<_>>();
        let res = data.iter().map_reduce(|n| *n * 2, |a, b| a + b).unwrap() / data.len();
        assert_eq!(res, 2);
    }
}
