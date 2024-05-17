/** ===============================================================================
* File: linalg.rs
* Author: Scott Stack
* Description: Implements some simple linear algebra types and methods
* ===============================================================================
*/

/// Generic 2D matrix type used in linear algebra
#[derive(Clone, Debug)]
pub struct Matrix<T> {
    data : Vec<T>,          // Internal flat representation of the matrix
    nrows : usize,          // Number of rows
    ncols : usize,          // Number of columns
}

/// Implementation of Matrix containing isize types
impl Matrix<isize>
{

    /// Return a matrix initialized with all zero data
    pub fn new_isize(nrows : usize, ncols : usize) -> Matrix<isize> {
        let temp_mat = Matrix::<isize> {
            data : vec![0; nrows * ncols],
            nrows : nrows,
            ncols : ncols,
        };
        return temp_mat;
    }

    /// Get a row/col from the matrix
    pub fn get(&self, row : usize, col : usize) -> isize
    {
        if (row >= self.nrows) || (col >= self.ncols) {
            panic!("Error: invalid row/col access to matrix!");
        }
        return self.data[row * self.ncols + col];
    }

    /// Set a single entry in the matrix
    pub fn set(&mut self, row : usize, col : usize, val : isize) { 
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

    /// Initialize matrix from a flat array
    pub fn set_from_arr(&mut self, vals : &[isize]) {
        for row in 0..self.nrows {
            for col in 0..self.ncols {
                self.set(row, col, vals[row * self.ncols + col])
            }
        }
    }

    /// Initialize matrix with zeros for isize matrices
    pub fn set_zeros(&mut self) {
        for i in 0..self.get_nrows() {
            for j in 0..self.get_ncols() {
                self.set(i, j, 0);
            }
        }
    }
}


/// Multiply a vector by a matrix for isize type
pub fn mat_vec_mult(mat : &Matrix<isize>, vec : &Vec<isize>) -> Vec<isize>
{
    if mat.get_ncols() != vec.len() {
        println!("mat cols = {} vec len = {}", mat.get_ncols(), vec.len());
        panic!("Invalid matrix/vector multiplication. Vector length does not match matrix cols");
    }

    // Perform multiplication
    let mut result : Vec<isize> = vec![0; mat.get_nrows()];
    for i in 0..mat.get_nrows() {
        for j in 0..mat.get_ncols() {
            result[i] += mat.get(i, j) * vec[j];
        }
    }

    // Transfer ownership of new vector to caller
    return result;
}


pub fn test_linalg() {
    let x = [1, 3, 4, 1, 1, 1, 3, 5 ,6];

    // Create 3x3 matrix
    let mut testmat : Matrix::<isize> = Matrix::<isize>::new_isize(3, 3);
    let testvec : Vec::<isize> = vec![1,2,3];

    testmat.set_from_arr(&x);

    let res = mat_vec_mult(&testmat, &testvec);
    println!("{:?} * {:?} = {:?}", testmat, testvec, res);
}


