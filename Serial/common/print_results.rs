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

pub fn print_results( name: &str, class_npb: &char, n1: &i64, n2: &i64, n3: &i64,
	    	          niter: &i64, t: &f64, mops: &f64, optype: &str,
	    	          passed_verification: &bool, totalthreads: &str )
{
	let size =
	    if (name == "IS") && (*n3 == 0) {
	        (n1 * (if *n2 != 0 { n2 } else { &1 })).to_string() }
	    else if (*n2 == 0) && (*n3 == 0) {
	        if name == "EP" { (1i128 << n1).to_string() }
	        else { n1.to_string() } }
	    else {
	        format!("{:>4}x{:>4}x{:>4}", n1, n2, n3) };
	let verification_status: String =
	        /*if passed_verification < &0 { String::from("NOT PERFORMED") }
	        else if passed_verification > &0 { String::from("SUCCESSFUL") }
	        else { String::from("UNSUCCESSFUL") }; */
	        if *passed_verification { String::from("SUCCESSFUL") }
	        else { String::from("UNSUCCESSFUL") };
    let rustc: &str =
            match option_env!("RUSTC") { Some(v) => v, None => "rustc", };
    let rustc_version: String = rustc_version::version().unwrap().to_string();
    let authors: String =
            env!("CARGO_PKG_AUTHORS").to_string()
                    .replace(":", "\n                            ");
    let rustflags: &str =
            match option_env!("RUSTFLAGS")
            {
                Some(v) => v,
                None => "-C opt-level=3 -C debuginfo=0 -C lto=true",
            };

	println!("\n\n {name} Benchmark Completed\n");
	println!(" class_npb       = {:>30}", class_npb);
    println!(" Size            = {:>30}", size);
	println!(" Total threads   = {:>30}", totalthreads);
	println!(" Iterations      = {:>30}", niter);
	println!(" Time in seconds = {:>30}", t);
	println!(" Mop/s total     = {:>30}", mops);
	println!(" Operation type  = {:>30}", optype);
	println!(" Verification    = {:>30}", verification_status);
	println!(" Version         = {:>30}", env!("CARGO_PKG_VERSION"));
	println!(" Compile date    = {:>30}", pkg_compile_time::pkg_compile_date!());
	println!(" Compiler ver    = {:>30}", rustc_version);
	println!("\n Compile options:");
	println!("    RUSTC        = {:>30}", rustc);
	println!("    RUSTFLAGS    = {:>30}", rustflags);
	println!("    RAND         = {:>30}", "f64rand");

	println!("\n {:-<70}", "");
	println!(" {} is developed by: {authors}",
	        env!("CARGO_PKG_NAME").to_string().replace("-", " ").replace("_", " "));
	println!(" In case of questions or problems, please send an e-mail.");
	println!(" {:-<70}\n", "");
}
