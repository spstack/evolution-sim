/** ===============================================================================
* File: linalg.rs
* Author: Scott Stack
* Description: Implements some simple linear algebra types and methods
* ===============================================================================
*/
use rand::Rng;
use serde::{Deserialize, Serialize};

/// Generic 2D matrix type used in linear algebra
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Matrix<T> {
    data : Vec<T>,          // Internal flat representation of the matrix
    nrows : usize,          // Number of rows
    ncols : usize,          // Number of columns
}

/// Implementation of Matrix containing isize types
impl<T> Matrix<T>
where 
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
    T: std::ops::Mul<Output = T>,
    T: std::ops::AddAssign,
    T: std::default::Default,
    T: Copy,
    T: std::cmp::PartialOrd,
    T: rand::distributions::uniform::SampleUniform,
{

    /// Return a matrix initialized with all zero data
    pub fn new(nrows : usize, ncols : usize) -> Matrix<T> {
        let temp_mat = Matrix::<T> {
            data : vec![T::default(); nrows * ncols],
            nrows : nrows,
            ncols : ncols,
        };
        return temp_mat;
    }

    /// Return a matrix initialized with random data
    pub fn random(nrows : usize, ncols : usize, min_val : T, max_val : T) -> Matrix<T> {
        let mut rng = rand::thread_rng();
        let mut temp_mat = Matrix::<T> {
            data : vec![T::default(); nrows * ncols],
            nrows : nrows,
            ncols : ncols,
        };

        // randomly set each val
        for i in 0..temp_mat.nrows {
            for j in 0..temp_mat.ncols {
                temp_mat.set(i, j, rng.gen_range(min_val..=max_val));
            }
        }

        return temp_mat;
    }


    /// Get a row/col from the matrix
    pub fn get(&self, row : usize, col : usize) -> T 
    {
        if (row >= self.nrows) || (col >= self.ncols) {
            panic!("Error: invalid row/col access to matrix!");
        }
        return self.data[row * self.ncols + col];
    }

    /// Set a single entry in the matrix
    pub fn set(&mut self, row : usize, col : usize, val : T) { 
        self.data[row * self.ncols + col] = val;
    }

    /// Get number of rows
    pub fn get_nrows(&self) -> usize {
        return self.nrows;
    }

    /// Get number of cols
    pub fn get_ncols(&self) -> usize {
        return self.ncols;
    }

    /// Initialize matrix from a flat array - allow dead code because this isn't used yet
    #[allow(dead_code)] 
    pub fn set_from_arr(&mut self, vals : &[T]) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                self.set(row, col, vals[row * self.ncols + col])
            }
        }
    }


    // OPERATIONS

    /// Multiply a vector by this matrix
    #[allow(dead_code)] 
    pub fn mult_vec(&self, vec : &Vec<T>) -> Vec<T> {
        if self.get_ncols() != vec.len() {
            println!("mat cols = {} vec len = {}", self.get_ncols(), vec.len());
            panic!("Invalid matrix/vector multiplication. Vector length does not match matrix cols");
        }

        // Perform multiplication
        let mut result : Vec<T> = vec![T::default(); self.get_nrows()];
        for i in 0..self.get_nrows() {
            for j in 0..self.get_ncols() {
                result[i] += self.get(i, j) * vec[j];
            }
        }

        // Transfer ownership of new vector to caller
        return result;

    }

    /// Multiply 2 matrices
    pub fn mult(&self, other : &Matrix<T>) -> Matrix<T> {
        if self.get_ncols() != other.get_nrows() {
            panic!("Invalid Matrix multiplication. Sizes do not match");
        }

        let res_ncols = other.get_ncols(); 
        let res_nrows = self.get_nrows();

        let mut result : Matrix<T> = Matrix::<T> {
            data : vec![T::default(); res_ncols * res_nrows],
            nrows : res_nrows,
            ncols : res_ncols,
        };

        // Perform multiplication
        for res_i in 0..res_nrows {
            for res_j in 0..res_ncols {
                for k in 0..self.get_ncols() {
                    result.set(res_i, res_j,  result.get(res_i, res_j) + (self.get(res_i, k) * other.get(k, res_j))); 
                }
            }
        }

        // Transfer ownership of new vector to caller
        return result;

    }

    /// Add 2 matrices
    pub fn add(&self, other : &Matrix<T>) -> Matrix<T> {
        if self.ncols != other.ncols || self.nrows != other.nrows {
            panic!("Error: Attempted to add matrices with different dimensions");
        }

        // Create new result matrix
        let mut result : Matrix<T> = Matrix::<T> {
            data : vec![T::default(); self.ncols * self.nrows],
            nrows : self.nrows,
            ncols : self.ncols,
        };

        // Add component-wise
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                result.set(i, j, self.get(i,j) + other.get(i, j));
            }
        }

        // Transfer ownership to caller
        return result;
    }

    /// Subtract 2 matrices
    #[allow(dead_code)]
    pub fn subtract(&self, other : &Matrix<T>) -> Matrix<T> {
        if self.ncols != other.ncols || self.nrows != other.nrows {
            panic!("Error: Attempted to add matrices with different dimensions");
        }

        // Create new result matrix
        let mut result : Matrix<T> = Matrix::<T> {
            data : vec![T::default(); self.ncols * self.nrows],
            nrows : self.nrows,
            ncols : self.ncols,
        };

        // Subtract component-wise
        for i in 0..self.nrows {
            for j in 0..self.ncols {
                result.set(i, j, self.get(i,j) - other.get(i, j));
            }
        }

        // Transfer ownership to caller
        return result;
    }


}




/// Unit tests for the linalg library
#[cfg(test)]
mod linalg_test {
    use super::*;

    /// Test matrix multiplication
    #[test]
    fn test_mat_mult() {
        let a = Matrix::<isize> {
            data : vec![2,3,4,
                        5,6,7,
                        8,9,10],
            ncols : 3,
            nrows : 3,
        };
        let b = Matrix::<isize> {
            data : vec![2,3,
                        5,6,
                        8,9],
            ncols : 2,
            nrows : 3,
        };

        // Perform multiplication
        let c = a.mult(&b);

        let expected_output : [[isize; 2]; 3] = [[51,  60],
                                                 [96,  114],
                                                 [141, 168]];

        // Check that expected output (from another tool) matches matrix multiplication
        for i in 0..expected_output.len() {
            for j in 0..expected_output[0].len() {
                assert_eq!(expected_output[i][j], c.get(i, j));
            }
        }
    }


    /// Test matrix vector multiplication operation
    #[test]
    pub fn test_mat_vec_mult() {

        // Create 3x3 matrix using the `set_from_arr` API
        let data : Vec<isize> = vec![1, 3, 4, 1, 1, 1, 3, 5 ,6];
        let mut testmat : Matrix::<isize> = Matrix::<isize>::new(3, 3);
        let testvec : Vec::<isize> = vec![1,2,3];

        testmat.set_from_arr(&data);

        // Multiply the vector by matrix
        let res = testmat.mult_vec(&testvec);

        // Set expected result
        let expected_res = vec![19, 6, 31];

        // Check result vs expected
        for i in 0..expected_res.len() {
            assert_eq!(res[i], expected_res[i]);
        }
    }

    
} 