use std::ops::Index;
use rand::Rng;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter};
use std::fmt;
use serde::{Serialize, Deserialize};
use std::convert::TryFrom;

/// Enum encapsulating errors produced in the module.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// Error returned by the [`Derange::apply`] method.
    /// Returned if the size of the `source`, `destination` and the derangement itself are not all equal.
    /// Contains the source size, destination size and the derangement size.
    SizeMismatch(usize, usize, usize),

    /// Error returned by the [`Derange::try_from`] method.
    /// Returned if the slice does not constitute a valid permutation.
    /// Contains the offending value.
    BadPermutation(usize),

    /// Error returned by the [`Derange::try_from`] method.
    /// Returned if the slice contains a fixed point.
    /// Index of the fixed point.
    FixedPoint(usize),
}


/// A special [`std::result::Result`] by the module.
/// See [`ErrorKind`] for more information.
///
/// ['Result']: std::result::Result
/// ['Derange::ErrorKind']: crate::derangement::derange::ErrorKind
pub type Result<T> = std::result::Result<T, ErrorKind>;

/// The struct representing the derangement.
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Derange {
    _map: Vec<usize>,
}

impl Derange {

    /// Generate a random derangement with an order of `size`.
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

    ///Returns `Some` with the value that `i` maps to.
    ///
    /// Otherwise returns `None` if `i >= size`.
    pub fn get(&self, i: usize) -> Option<&usize> {
        self._map.get(i)
    }

    ///Get the inverse of this derangement as a new object.
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

    ///The underlying vector map as a slice. The map is represented by mapping `n` to the `n`th element.
    ///
    /// For example the derangement where 0 maps to 1, 1 maps to 2 and 2 maps to 0 would be represented with the array `[1, 2, 0]`
    pub fn map(&self) -> &[usize] {
        self._map.as_slice()
    }

    /// Apply the derangement to `source` and clone it into `destination`.
    ///
    /// If the apply succeeds, then a `Ok(())` is returned, and the permutation is applied to the destination slice. Otherwise an `Err(ErrorKind)` is returned and the destination slice is left alone.
    pub fn apply<T: Clone>(&self, source: &[T], destination: & mut [T]) -> Result<()> {

        if source.len() != destination.len() && source.len() != self._map.len() {
            return Result::Err(ErrorKind::SizeMismatch(source.len(), destination.len(), self._map.len()));
        }

        for (index, obj) in self._map.iter().zip(destination.iter_mut()) {
            *obj = source[*index].clone();
        }

        Ok(())

    }


}

impl Index<usize> for Derange {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl TryFrom<&[usize]> for Derange {
    type Error = ErrorKind;

    ///Attempts to convert a slice of indices into a derangement.
    ///
    /// The slice must represent a permutation with no fixed points, or the conversion will fail.
    fn try_from(value: &[usize]) -> Result<Self> {

        let mut clone = Vec::from(value);

        clone.sort();

        for (i, (original, cloned)) in value.iter().zip(clone.iter()).enumerate() {
            if i != *cloned {
                return Result::Err(ErrorKind::BadPermutation(*cloned));
            }

            if i == *original {
                return Result::Err(ErrorKind::FixedPoint(i))
            }


        }

        Ok(Derange {
            _map: Vec::from(value)
        })

    }
}

impl Display for Derange {

    ///Formats the derangement as a permutation in cyclic form.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        let mut checklist: Vec<_> = (0..self._map.len()).map(|_| false ).collect();

        loop {
            match checklist.iter().position(|x| *x == false) {
                Some(start) => {
                    let mut next = start;

                    f.write_str("(")?;

                    loop {
                        write!(f, "{}", next)?;

                        checklist[next] = true;

                        next = self._map[next];

                        if next == start {
                            f.write_str(")")?;
                            break;
                        }

                        f.write_str(" ")?;
                    }
                }
                None => {
                    break;
                }
            }
        }

        fmt::Result::Ok(())

    }
}
