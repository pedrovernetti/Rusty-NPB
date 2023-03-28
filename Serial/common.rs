///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//                                 Rusty NPB                                 //
//                             (Serial Version)                              //
//                                                                           //
//                                MIT License                                //
//                Copyright (C) 2023 Pedro Vernetti Gonçalves                //
//                                                                           //
//  Permission is hereby granted, free of charge, to any person obtaining a  //
// copy of this software and associated documentation files (the "Software"),//
// to deal in the Software without restriction, including without limitation //
//  the rights to use, copy, modify, merge, publish, distribute, sublicense, //
//   and/or sell copies of the Software, and to permit persons to whom the   //
//    Software is furnished to do so, subject to the following conditions:   //
//  The above copyright notice and this permission notice shall be included  //
//          in all copies or substantial portions of the Software.           //
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS  //
//        OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF         //
//   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  //
//    IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY   //
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT //
// OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR  //
//                 THE USE OR OTHER DEALINGS IN THE SOFTWARE.                //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//   The original NPB 3.4.1 version was written in Fortran and belongs to:   //
//                   http://www.nas.nasa.gov/Software/NPB/                   //
//                     Authors of the Fortran code are:                      //
//           P. O. Frederickson, D. H. Bailey, A. C. Woo, H. Jin             //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////



/* causing compilation to abort as soon as possible, if compilation is not
 * being done using cargo (env! won't find CARGO env variable)
 */
#[allow(dead_code)]
const CARGO: &str = env!("CARGO");



mod print_results;
pub use self::print_results::print_results as rs_print_results;

pub mod f64rand;



pub use std::time::{Duration, Instant};

// returns time since 'i' in microseconds
pub fn elapsed( i: &Instant ) -> u128
{
    return i.elapsed().as_micros();
}



pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CLASS: char = env!("CLASS").as_bytes()[0] as char;
