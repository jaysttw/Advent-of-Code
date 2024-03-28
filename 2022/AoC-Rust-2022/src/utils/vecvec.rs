// use itertools::Either;
use std::fmt;

// pub mod vecvec;

#[derive(Debug)]
pub enum VecVecError<T> {
    Empty(EmptyVectorError<T>),
    UnevenLength(UnevenVectorError<T>),
}

impl<T> From<EmptyVectorError<T>> for VecVecError<T> {
    fn from(err: EmptyVectorError<T>) -> VecVecError<T> {
        VecVecError::Empty(err)
    }
}

impl<T> From<UnevenVectorError<T>> for VecVecError<T> {
    fn from(err: UnevenVectorError<T>) -> VecVecError<T> {
        VecVecError::UnevenLength(err)
    }
}

#[derive(Debug)]
struct EmptyVectorError<T> {
    culprit: Vec<Vec<T>>,
}

impl<T: std::fmt::Debug> fmt::Display for EmptyVectorError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Empty vector in vecvec rotation. Culprit: {:?}", self.culprit)
    }
}

impl<T> EmptyVectorError<T> {
    fn new(culprit: Vec<Vec<T>>) -> EmptyVectorError<T> {
        EmptyVectorError {
            culprit : culprit,
        }
    }
}

#[derive(Debug)]
struct UnevenVectorError<T> {
    culprit: Vec<Vec<T>>,
    lengths: Vec<usize>,
}

impl<T: std::fmt::Debug> fmt::Display for UnevenVectorError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Vector of vectors of uneven length in vecvec rotation. \nLengths: {:?} \nCulprit: {:?}",
            self.lengths,
            self.culprit,
        )
    }
}

impl<T> UnevenVectorError<T> {
    fn new(
        culprit: Vec<Vec<T>>,
        lengths: Vec<usize>,
    ) -> UnevenVectorError<T> {
        UnevenVectorError {
            culprit : culprit,
            lengths : lengths,
        }
    }
}

fn reverse_vector_iter<T>(
    vec: Vec<T>,
    reverse: bool,
) -> Vec<T>
where T: Clone {
// ) -> Box<dyn Iterator<Item = T>> {
// ) -> impl Iterator<Item = T> {
    // let result: Iterator<Item=T> = vec.iter();
    // let result: Box<dyn Iterator<Item = T>> = if reverse {
    let result: Vec<T> = if reverse {
        vec.iter().map(|i| i.clone()).rev().collect::<Vec<T>>()
    } else {
        vec
    };
    result
}

pub fn rotate_vecvec<T>(
    vecvec: Vec<Vec<T>>,
    clockwise: bool,
) -> Result<Vec<Vec<T>>, VecVecError<T>>
where T: Clone {
    // Does not support Vec of uneven Vec lengths yet. May suuport in future,
    // but throw error for now.
    let lengths_of_input: Vec<usize> = vecvec.iter().map(|v| v.len()).collect();
    if lengths_of_input.iter().min() != lengths_of_input.iter().max() {
        return Err(
            VecVecError::UnevenLength(
                UnevenVectorError::new(
                    vecvec,
                    lengths_of_input,
                )
            )
        )
    }

    if lengths_of_input.contains(&0) {
        return Err(VecVecError::Empty(EmptyVectorError::new(vecvec)))
    }

    let iter_range: Vec<usize> = reverse_vector_iter(
        (0..lengths_of_input[0]).collect::<Vec<usize>>(),
        !clockwise,
    );// .iter().collect::<Vec<usize>>();
    // .to_vec();

    let result: Vec<Vec<T>> = iter_range.iter()
    .map(
        |i| reverse_vector_iter(vecvec.clone(), clockwise).to_vec().iter().map(
            |inner| inner[*i].clone()
        ).collect::<Vec<T>>()
    ).collect();

    Ok(result)
}