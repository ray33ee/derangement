pub mod derange;

#[cfg(test)]
mod tests {
    use rand_xoshiro::{Xoshiro128StarStar};
    use rand::SeedableRng;
    use crate::derange::Derange;

    #[test]
    fn small_test() {

        let mut rng = Xoshiro128StarStar::seed_from_u64(0);

        let derange = Derange::new(& mut rng, 5);

        println!("derange: {:?}", derange);

        assert_eq!(derange.map(), [1, 3, 4, 0, 2]);

    }

    #[test]
    fn big_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(1);

        let derange = Derange::new(& mut rng, 16);

        println!("derange: {:?}", derange);

        assert_eq!(derange.map(), [5, 12, 6, 2, 8, 1, 9, 10, 13, 11, 15, 14, 7, 0, 3, 4]);
    }

    #[test]
    fn get_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(2);

        let derange = Derange::new(& mut rng, 5);

        let result = [2, 0, 1, 4, 3];

        assert_eq!(result, derange.map());

        for i in 0..5 {
            assert_eq!(derange[i], result[i]);
        }
    }

    #[test]
    fn inverse_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(3);

        let derange = Derange::new(& mut rng, 10);

        let inverse = derange.inverse();

        for i in 0..10 {
            println!("i: {}", i);
            assert_eq!(derange[inverse[i]], i);
            assert_eq!(inverse[derange[i]], i);
        }
    }
}
