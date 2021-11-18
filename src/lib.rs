pub mod derange;

#[cfg(test)]
mod tests {
    use rand_xoshiro::{Xoshiro128StarStar};
    use rand::SeedableRng;
    use crate::derange::Derange;
    use std::convert::TryFrom;

    use crate::derange::ErrorKind;


    #[test]
    fn small_test() {

        let mut rng = Xoshiro128StarStar::seed_from_u64(0);

        let derange = Derange::new(& mut rng, 5);

        assert_eq!(derange.map(), [1, 3, 4, 0, 2]);

    }

    #[test]
    fn big_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(1);

        let derange = Derange::new(& mut rng, 16);

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
            assert_eq!(derange[inverse[i]], i);
            assert_eq!(inverse[derange[i]], i);
        }
    }

    #[test]
    fn apply_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(4);

        let derange = Derange::new(& mut rng, 5);

        let alphab = ['a', 'b', 'c', 'd', 'e'];

        let mut alphac = ['\0', '\0', '\0', '\0', '\0'];

        derange.apply(&alphab[..], &mut alphac[..]).unwrap();

        assert_eq!(alphac, ['c', 'e', 'a', 'b', 'd'])
    }

    #[test]
    fn display_test() {
        let mut rng = Xoshiro128StarStar::seed_from_u64(6);

        let derange = Derange::new(& mut rng, 10);

        assert_eq!(derange.to_string(), String::from("(0 9 5 7)(1 6 3)(2 8 4)"));

    }

    #[test]
    fn try_from_test() {

        //An array representing a valid derangement
        let slice = [2, 3, 0, 1];

        let derange = Derange::try_from(&slice[..]).unwrap();

        assert_eq!(derange.map(), slice);

        // A valid permutation but not a derangement
        let slice = [0, 3, 2, 1];

        if let Result::Err(ErrorKind::FixedPoint(0)) = Derange::try_from(&slice[..]) {

        } else {
            panic!("Expected fixed point (at 0) error");
        }

        //Not even a valid permutation
        let slice = [2, 7, 1, 0];

        if let Result::Err(ErrorKind::BadPermutation(7)) = Derange::try_from(&slice[..]) {

        } else {
            panic!("Expected bad permutation (7) error");
        }
    }

}
