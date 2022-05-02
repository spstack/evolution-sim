/** ===============================================================================
 * File: matrix_math.rs
 * Author: Scott Stack
 * Created: 5/1/2022
 * Description: Implements common linear algebra math functions
 * ===============================================================================*/



// Multiply 2 matrices a * b
// pub fn matrix_mult<T, ARows, ACols, BRows, BCols>(output : &[[T; BCols]; ARows], mat_a : &[[T; ACols]; ARows], mat_b : &[[T; BCols]; BRows]) {
//     println!("a len = {} b len = {}!", mat_a.len(), mat_b.len());
// }

// // Multiply a matrix and vector 
// pub fn matrix_vec_mult<T, R, C>(output : &[T; R], mat_in : &[[T; C]; R], vec_in : &[T; C]) {
//     println!("a len = {} b len = {}!", mat_in.len(), vec_in.len());
// }


pub fn matrix_mult<T, const ARows : usize, const ACols : usize, const BRows : usize, const BCols : usize>( mut output : &[[T; BCols]; ARows], mat_a : &[[T; ACols]; ARows], mat_b : &[[T; BCols]; BRows]) {
    println!("a len = {} b len = {}!", mat_a.len(), mat_b.len());
}

// Multiply a matrix and vector 
pub fn matrix_vec_mult<T, const RMax : usize, const CMax : usize>(mut output : &[T], mat_in : &[[T; CMax]], vec_in : &[T]) {
    println!("a len = {} b len = {}!", mat_in.len(), vec_in.len());

}


// /// Matrix vector multiplication where the matrix is specified as a flat slice
// pub fn matrix_vec_mult_flat(mut output : &[isize], mat_in : &[isize], vec_in : &[isize], rows : usize, cols : usize) {
//     println!("mat in total len = {} vec_in len = {}. rows = {} cols = {}", mat_in.len(), vec_in.len(), rows, cols);

//     let mat_in_rows
    
//     for row in 0..rows {
//         output[row] = 0;
//         for col in 0..cols {
//             output[row] += mat_in[]
//         }
//     }

// }

// /// Matrix vector multiplication where the matrix is specified as a flat slice
// pub fn matrix_vec_mult_flat(output : &[isize], mat_in : &[isize], vec_in : &[isize], rows : usize, cols : usize, col_len : usize) {
//     println!("mat in total len = {} vec_in len = {}. rows = {} cols = {}", mat_in.len(), vec_in.len(), rows, cols);
    
//     for row in 0..rows {
//         output[row] = 0;
//         for col in 0..cols {
//             output[row] += mat_in[row * col_len + cols] * vec_in[col];
//             // output[row] += (mat[row,col] * vec_in[col]);
//         }
//     }

// }
