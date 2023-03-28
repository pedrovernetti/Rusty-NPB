///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//                                 Rusty NPB                                 //
//                     (Threads & Channels Version - TC)                     //
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



#![allow(non_upper_case_globals)]

const r23: f64 = 0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5*0.5;
const r46: f64 = r23 * r23;
const t23: f64 = 2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0*2.0;
const t46: f64 = t23 * t23;



pub fn randlc( x: &mut f64, a: &f64 ) -> f64
{
    /* This routine returns a uniform pseudorandom double precision number in the
     * range (0, 1) by using the linear congruential generator
     *
     * x_{k+1} = a x_k  (mod 2^46)
     *
     * where 0 < x_k < 2^46 and 0 < a < 2^46.  This scheme generates 2^44 numbers
     * before repeating.  The argument A is the same as 'a' in the above formula,
     * and X is the same as x_0.  A and X must be odd double precision integers
     * in the range (1, 2^46).  The returned value RANDLC is normalized to be
     * between 0 and 1, i.e. RANDLC = 2^(-46) * x_1.  X is updated to contain
     * the new seed x_1, so that subsequent calls to RANDLC using the same
     * arguments will generate a continuous sequence.
     *
     * This routine should produce the same results on any computer with at least
     * 48 mantissa bits in double precision floating point data.  On 64 bit
     * systems, double precision should be disabled.
     *
     * (original c code and comments by David H. Bailey)
     */

    let (mut t1, t2, t3, t4, a1, a2, x1, x2, z):
            (f64, f64, f64, f64, f64, f64, f64, f64, f64);

    // breaking A into two parts such that A = 2^23 * A1 + A2
    t1 = r23 * a;
    a1 = t1.round();
    a2 = a - t23 * a1;

    // breaking X into two parts such that X = 2^23 * X1 + X2
    t1 = r23 * *x;
    x1 = t1.round();
    x2 = *x - t23 * x1;
    t1 = a1 * x2 + a2 * x1;
    t2 = (r23 * t1).round();

    // computing Z = A1 * X2 + A2 * X1  (mod 2^23)
    z = t1 - t23 * t2;
    t3 = t23 * z + a2 * x2;
    t4 = (r46 * t3).round();

    // computing X = 2^23 * Z + A2 * X2  (mod 2^46)
    *x = t3 - (t23 * t23) * t4;

    return r46 * *x;
}



pub fn nrandlc( n: &usize, x: &mut f64, a: &f64, y: &mut [f64] )
{
    /* This routine generates N uniform pseudorandom double precision numbers in
     * the range (0, 1) by using the linear congruential generator
     *
     * x_{k+1} = a x_k  (mod 2^46)
     *
     * where 0 < x_k < 2^46 and 0 < a < 2^46.  This scheme generates 2^44 numbers
     * before repeating.  The argument A is the same as 'a' in the above formula,
     * and X is the same as x_0.  A and X must be odd double precision integers
     * in the range (1, 2^46).  The N results are placed in Y and are normalized
     * to be between 0 and 1.  X is updated to contain the new seed, so that
     * subsequent calls to VRANLC using the same arguments will generate a
     * continuous sequence.  If N is zero, only initialization is performed, and
     * the variables X, A and Y are ignored.
     *
     * This routine is the standard version designed for scalar or RISC systems.
     * However, it should produce the same results on any single processor
     * computer with at least 48 mantissa bits in double precision floating point
     * data.  On 64 bit systems, double precision should be disabled.
     */

    let mut x_copy: f64 = *x;
    let (mut t1, mut t2, mut t3, mut t4, a1, a2, mut x1, mut x2, mut z):
            (f64, f64, f64, f64, f64, f64, f64, f64, f64);

    // breaking A into two parts such that A = 2^23 * A1 + A2
    t1 = r23 * a;
    a1 = t1.round();
    a2 = a - t23 * a1;

    // generating N results
    for i in 1usize..=*n
    {
        // breaking X into two parts such that X = 2^23 * X1 + X2
        t1 = r23 * x_copy;
        x1 = t1.round();
        x2 = x_copy - t23 * x1;
        t1 = a1 * x2 + a2 * x1;
        t2 = (r23 * t1).round();

        // computing Z = A1 * X2 + A2 * X1  (mod 2^23)
        z = t1 - t23 * t2;
        t3 = t23 * z + a2 * x2;
        t4 = (r46 * t3).round();

        // computing X = 2^23 * Z + A2 * X2  (mod 2^46)
        x_copy = t3 - t46 * t4;

        y[i] = r46 * x_copy;
    }

    *x = x_copy;
}
