///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//                                 Rusty NPB                                 //
//                     (Threads & Channels Version - TC)                     //
//                                                                           //
//               Rust version of the APP Benchmark 1, the "EP",              //
//                   or "embarassingly parallel" benchmark.                  //
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

///////////////////////////////////////////////////////////////////////////////
//                                                                           //
//    This code implements the random-number generator described in the      //
//    NAS Parallel Benchmark document RNR Technical Report RNR-94-007.       //
//    The code is "embarrassingly" parallel in that no communication is      //
//    required for the generation of the random numbers itself.              //
//                                                                           //
///////////////////////////////////////////////////////////////////////////////



#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

use std::env;

use std::thread;
use std::sync::mpsc;

mod common;
use common::*;



fn verify( M: &usize, sx: &f64, sy: &f64 ) -> bool
{
    const EPSILON: f64  = 1.0e-8; // random numbers precision

	let (sx_verify_value, sy_verify_value): (f64, f64);

	if *M == 24
	{
		sx_verify_value = -3.247834652034740e+3;
		sy_verify_value = -6.958407078382297e+3;
	}
	else if *M == 25
	{
		sx_verify_value = -2.863319731645753e+3;
		sy_verify_value = -6.320053679109499e+3;
	}
	else if *M == 28
	{
		sx_verify_value = -4.295875165629892e+3;
		sy_verify_value = -1.580732573678431e+4;
	}
	else if *M == 30
	{
		sx_verify_value =  4.033815542441498e+4;
		sy_verify_value = -2.660669192809235e+4;
	}
	else if *M == 32
	{
		sx_verify_value =  4.764367927995374e+4;
		sy_verify_value = -8.084072988043731e+4;
	}
	else if *M == 36
	{
		sx_verify_value =  1.982481200946593e+5;
		sy_verify_value = -1.020596636361769e+5;
	}
	else if *M == 40
	{
		sx_verify_value = -5.319717441530e+05;
		sy_verify_value = -3.688834557731e+05;
	}
	else
	{
		return false;
	}

	return (((sx - sx_verify_value).abs() / sx_verify_value) <= EPSILON) &&
	       (((sy - sy_verify_value).abs() / sy_verify_value) <= EPSILON);
}



fn main()
{
    const M:       usize = // log2 of the number of complex pairs of (0, 1) unif. random numbers
            if      CLASS == 'S' { 24 }
            else if CLASS == 'W' { 25 }
            else if CLASS == 'A' { 28 }
            else if CLASS == 'B' { 30 }
            else if CLASS == 'C' { 32 }
            else if CLASS == 'D' { 36 }
            else if CLASS == 'E' { 40 }
            else                 { 00 };

    const MK:      usize = 16; // log2 of the size of each batch of random numbers
    const NK:      usize = 1 << MK;
    const NQ:      usize = 10;
    const A:       f64   = 1220703125.0;
    const S:       f64   = 271828183.0;
    const NK_PLUS: usize = (2 * NK) + 1; // 2NK + 1
    const SIZE:    i128  = 1 << (M + 1); // total number of random nums. to be generated

    /* number of "batches" of random number pairs generated.
     * (adjust if the processors count isn't evenly divisible by the total number)
     */
    const TOTAL_BATCHES: usize = 1 << (M - MK); // 2^(M-MK)

	/* ********************************************************************* */

    // processing command line arguments
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();
    let mut use_timers: bool = false;
    for arg in args
    {
        if (arg == "--with-timers") || (arg == "-t") { use_timers = true; }
    }

	/* ********************************************************************* */

    // variables
	let (mut t1, mut t2, an, mut Mops): (f64, f64, f64, f64);
	let (mut sx, mut sy, mut gc): (f64, f64, f64) = (0.0, 0.0, 0.0);
	let /*mut*/ nit: i64;
    let (mut dum0, mut dum1, mut dum2): (f64, f64, [f64; 1]) = (1.0, 1.0, [1.0; 1]);
    let mut x: [f64; NK_PLUS] = [0.0; NK_PLUS];
    let mut q: [f64; NQ] = [0.0; NQ];
    let available_threads: usize = thread::available_parallelism().unwrap().get();
    let (mut time0, mut time1, mut time2): (u128, u128, u128) = (0, 0, 0);

	/* ********************************************************************* */

    // beginning message
	println!("\n\n {} {PACKAGE_VERSION}-TC - EP Benchmark\n",
	        PACKAGE_NAME.to_string().replace("-", " ").replace("_", " "));
	println!(" Number of random numbers generated: {}", SIZE);
	println!(" Number of available threads:        {}", available_threads);

    /* Call the random number generator functions and initialize
     * the x-array to reduce the effects of paging on the timings.
     * Also, call all mathematical functions that are used. Make
     * sure these initializations cannot be eliminated as dead code.
    */
    f64rand::nrandlc(&0, &mut dum0, &dum1, &mut dum2);
    dum2[0] = f64rand::randlc(&mut dum1, &dum0);
    for i in 0usize..NK_PLUS { x[i] = -1.0e99; }
    Mops = std::hint::black_box(f64::max(x[NK], dum2[0]).abs().sqrt().ln());

    // starting main timer
    let start_moment = Instant::now();

	/* ********************************************************************* */

	t1 = A;
	f64rand::nrandlc(&0, &mut t1, &A, &mut x);

	// compute AN = A ^ (2 * NK) (mod 2^46)
	t1 = A;
	for _i in 0usize..(MK + 1)
	{
	    t2 = t1;
	    f64rand::randlc(&mut t1, &t2);
	}
	an = t1;

	/* ********************************************************************* */

    /* main parallelism */

    let batches_per_thread: usize = TOTAL_BATCHES / available_threads;
   	let mut threads: Vec<thread::JoinHandle<()>> = Vec::with_capacity(available_threads);
    let (sxsy_sender, sxsy_receiver):
            (mpsc::Sender<(f64, f64)>, mpsc::Receiver<(f64, f64)>) = mpsc::channel();
    let (times_sender, times_receiver):
            (mpsc::Sender<(u128, u128)>, mpsc::Receiver<(u128, u128)>) = mpsc::channel();
    let (q_sender, q_receiver):
            (mpsc::Sender<[f64; NQ]>, mpsc::Receiver<[f64; NQ]>) = mpsc::channel();
   	for i in 0usize..available_threads
   	{
        let local_sxsy_sender = sxsy_sender.clone();
        let local_timer_sender = times_sender.clone();
        let local_q_sender = q_sender.clone();
        let current_batch = i * batches_per_thread;
        threads.push(thread::spawn(move ||
        {
            let mut q: [f64; NQ] = [0.0; NQ];
            let mut x: [f64; NK_PLUS] = [0.0; NK_PLUS];
	        let (mut x1, mut x2, mut t1, mut t2, mut t3, mut t4): (f64, f64, f64, f64, f64, f64);
	        let (mut sx, mut sy): (f64, f64) = (0.0, 0.0);
            let (k_offset, mut kk, mut ik): (i64, i64, i64);
            let mut start_moment = Instant::now();

	        /* each instance of the following loop may be performed independently.
	         * we compute the k offsets separately to take into account the fact
	         * that some nodes have more numbers to generate than others
	         */
	        k_offset = -1;

	        for k in (current_batch + 1)..=(current_batch + batches_per_thread)
	        {
		        kk = k_offset + (k as i64);
		        t1 = S;
		        t2 = an;

		        if use_timers { start_moment = Instant::now(); }

		        /* find starting seed t1 for this kk */
		        for _i in 1usize..=100usize
		        {
			        ik = kk / 2;
			        if (2 * ik) != kk { t3 = f64rand::randlc(&mut t1, &t2); }
			        if ik == 0 { break; }
			        t4 = t2;
			        t3 = f64rand::randlc(&mut t2, &t4);
			        kk = ik;
		        }

		        /* compute uniform pseudorandom numbers */
		        f64rand::nrandlc(&(2 * NK), &mut t1, &A, &mut x);
                if use_timers { time2 += elapsed(&start_moment); }

		        /* compute gaussian deviates by acceptance-rejection method and
		         * tally counts in concentric square annuli. this loop is not
		         * vectorizable.
		         */
		        if use_timers { start_moment = Instant::now(); }
		        for i in 0usize..NK
		        {
			        x1 = 2.0 * x[2 * i] - 1.0;
			        x2 = 2.0 * x[(2 * i) + 1] - 1.0;
			        t1 = (x1 * x1) + (x2 * x2);
			        if t1 <= 1.0
			        {
				        t2 = (-2.0 * t1.ln() / t1).sqrt();
				        t3 = x1 * t2; // Xi
				        t4 = x2 * t2; // Yi
				        q[f64::max(t3.abs(), t4.abs()) as usize] += 1.0; // count
				        sx += t3; // sum of Xi
				        sy += t4; // sum of Yi
			        }
		        }
		        if use_timers { time1 += elapsed(&start_moment); }
		        local_sxsy_sender.send((sx, sy)).unwrap();
	        }

	        if use_timers { local_timer_sender.send((time1, time2)).unwrap(); }
            local_q_sender.send(q).unwrap();
        }));
    }

    while let Some(current_thread) = threads.pop() { current_thread.join().unwrap(); }

    /* reducing all data */

    while let Ok(another_q) = q_receiver.try_recv()
    {
        for i in 0usize..NQ { q[i] += another_q[i]; }
    }

	for i in 0usize..NQ { gc += q[i]; }
    while let Ok(sxsy) = sxsy_receiver.try_recv() { sx += sxsy.0; sy += sxsy.1; }

    if use_timers
    {
        while let Ok(times) = times_receiver.try_recv() { time1 += times.0 ; time2 += times.1; }
    }

	/* ********************************************************************* */

    // computing final stats
    if use_timers
    {
        time1 /= available_threads as u128;
        time2 /= available_threads as u128;
    }
    time0 += elapsed(&start_moment);
	nit = 0; // ?
	let verified = verify(&M, &sx, &sy);
	Mops = (f64::powf(2.0, (M + 1) as f64) / ((time0 as f64) / 1e6)) / 1e6;
	        // (2^(M+1) / secs) / 1000000

	/* ********************************************************************* */

    // EP-specific results summary
	println!("\n EP Benchmark Results:\n");
	println!(" CPU Time           =                {:.4}", ((time0 as f64) / 1e6));
	println!(" N                  =                2^{}", M);
	println!(" No. Gaussian Pairs =                {:.0}", gc);
	println!(" Sums               =                {:.15}", sx);
	println!("                                     {:.15}", sy);
	println!(" Counts:");
	for i in 0usize..(NQ - 1) { println!("    {}    {:>15}", i, q[i] as i64); }

    // NPB benchmark results
	rs_print_results("EP", &CLASS, &((M as i64) + 1), &0, &0, &nit, &((time0 as f64) / 1e6),
                     &Mops, "Random numbers generated", &verified, &available_threads.to_string());

    // timers' results
	if use_timers
	{
		println!(" Total time:     {:.6} (100.00%)", ((time0 as f64) / 1e6));
		println!(" Gaussian pairs: {:.6} ({:>6.2}%)",
		        ((time1 as f64) / 1e6), ((time1 as f64) / (time0 as f64)) * 100.0);
		println!(" Random numbers: {:.6} ({:>6.2}%)",
		        ((time2 as f64) / 1e6), ((time2 as f64) / (time0 as f64)) * 100.0);
	}
}
