use std::ops::Index;
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Derange {
    _map: Vec<usize>,
}

impl Derange {

    /// Generate a random derangement with an order of `size`
    pub fn new<R: Rng + ?Sized>(rng: & mut R, size: usize) -> Self {

        let mut permutation  = Vec::with_capacity(size);
        let mut derangement = Vec::with_capacity(size);

        // Fill the permutation with 0..STATE_SIZE
        for i in 0..size {
            permutation.push(i);
            derangement.push(0);
        }

        //Shuffle the permutation
        permutation.shuffle( rng);

        //Get a slice representing the remaining elements to partition
        let mut remaining = &permutation[..];

        // Randomly partition and convert from cyclic to permutation notation
        while remaining.len() != 0 {

            //If there are only 2 elements left, the partition size MUST be 2. Otherwise randomly get the partition size
            let mut partition_size = if remaining.len() == 2
            {
                2
            }
            else {
                rng.gen_range(2..=remaining.len()-1)
            };

            //Turn remaining.len()-1 into remaining.len() since remaining.len()-1 cannot be a partition size (it would introduce a fixed point)
            if partition_size == remaining.len()-1 { //Special case
                partition_size = remaining.len();
            }

            //Get a slice representing the partition
            let partition = &remaining[..partition_size];

            //Iterate over the partiton, treating it as a cycle and converting it into standard form
            for (i, element) in partition.iter().enumerate() {
                derangement[*element] = partition[(i+1) % partition_size];
            }

            //Move the slice forward
            remaining = &remaining[partition_size..];
        }

        Derange {
            _map: derangement,
        }
    }

    ///Return the value that `i` maps to
    pub fn get(&self, i: usize) -> Option<&usize> {
        self._map.get(i)
    }

    ///Get the inverse of this derangement, which is itself also a derangement
    pub fn inverse(&self) -> Derange {

        let mut inverse = Vec::with_capacity(self._map.len());

        for _ in 0..self._map.len() {
            inverse.push(0);
        }

        for (i, val) in self._map.iter().enumerate() {
            inverse[*val] = i;
        }

        Derange {
            _map: inverse,
        }
    }

    ///The underlying vector map as a slice. The map is represented by mapping `n` to the `n`th element
    pub fn map(&self) -> &[usize] {
        self._map.as_slice()
    }

}

impl Index<usize> for Derange {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}


