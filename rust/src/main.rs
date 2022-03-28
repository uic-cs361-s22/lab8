use getopt::Opt;
use sysconf::raw::{sysconf, SysconfVariable};
use scheduler::{CpuSet, set_affinity};
use std::thread::{spawn, JoinHandle};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use std::time::Instant;

const MAX_ARRAY_SIZE: usize = 1000000000;
static mut GRAND_SUM: i64 = 0;

lazy_static!{
	static ref GRAND_SUM_WITH_LOCK: Mutex<i64> = Mutex::new(0);
}

fn mode0(vec: Arc<Vec<i64>>) {
	for i in 0..vec.len() {
		unsafe {
			GRAND_SUM += vec[i];
		}
	}
}

fn mode1(start_idx: usize, slice_size: usize, vec: Arc<Vec<i64>>) {
	for i in start_idx..start_idx+slice_size {
		unsafe {
			GRAND_SUM += vec[i];
		}
	}
}

fn mode2(start_idx: usize, slice_size: usize, vec: Arc<Vec<i64>>) {
	//To-Do 3: Complete the function body for mode 2
}

fn mode3(start_idx: usize, slice_size: usize, vec: Arc<Vec<i64>>) {
	//To-Do 4: Complete the function body for mode 3
}

fn cores_extraction(arg: String, max_cores: usize, cores: &mut Vec<usize>) {
	let tokens = arg.split(",");

	for token in tokens {
		let core = token.parse::<usize>().unwrap() - 1;// Subtracting -1 to compensate 0 indexing
		if core>max_cores {
			println!("Requested core {} not within range", core);
		}
		cores.push(core);
	}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut opts = getopt::Parser::new(&args, "ht:c:n:m:");
	let max_cores = sysconf(SysconfVariable::ScNprocessorsOnln).unwrap() as usize;

	let mut threads_count: usize = 1;
    let mut array_size: usize = 100;
    let mut mode: i32 = 0;
	let mut cores_permitted: Vec<usize> = Vec::with_capacity(max_cores);

    loop {
        match opts.next().transpose().unwrap() {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => {
                    println!("Options:");
                    println!("t: Threads Count");
                    println!("c: Cores Count");
                    println!("n: Array Size");
                    println!("m: Mode");
                    println!("   Mode 0: 1 thread no lock");
                    println!("   Mode 1: Multi-threads no lock");
                    println!("   Mode 2: Multi-threads with lock");
                    println!("   Mode 3: Multi-threads with lock and grouped sum");
                    return;
                }
                Opt('t', Some(optarg)) => {
                    threads_count = optarg.parse::<usize>().unwrap();
                }
                Opt('c', Some(optarg)) => {
                    cores_extraction(optarg, max_cores, &mut cores_permitted);
                }
                Opt('n', Some(optarg)) => {
                    array_size = optarg.parse::<usize>().unwrap();
                    if array_size > MAX_ARRAY_SIZE {
                        println!("Array size cannot be greater than {}", MAX_ARRAY_SIZE);
                        return;
                    }
                }
                Opt('m', Some(optarg)) => {
                    mode = optarg.parse::<i32>().unwrap();
                    if mode<0 || mode>3 {
                        println!("Invalid mode, check out help with -h option");
                        return;
                    }
                }
                _ => { println!("Unknown option, look out for the help"); return; }
            }
        }
    }

	// Set core affinity
	let mut cpu_set = CpuSet::new(max_cores);
	if cores_permitted.len() == 0 {
		cores_permitted.push(0);
	}
	let num_cores = cores_permitted.len();
	for core in cores_permitted.into_iter() {
		cpu_set.set(core);
	}
	set_affinity(0, cpu_set).unwrap();
	
	// Print configuration
	println!("\n# CONFIGURATION #");
	println!("Threads Count: {}", threads_count);
	println!("Cores Count: {}", num_cores);
	println!("Array Size: {}", array_size);
	println!("Mode : {}", mode);

	// Populate the Array,
	println!("\nPopulating the Array...");
	let array_ref: Arc<Vec<i64>>;
	let mut array: Vec<i64> = Vec::new();
	for i in 1..(array_size as i64) + 1 {
		array.push(i);
	}
	array_ref = array.into();

	println!("Running the experiment");
	let now = Instant::now();
	if mode == 0 {
		mode0(array_ref);
	} else {

		// Create worker threads
		let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(threads_count);
		let slice_size = array_size / threads_count; /* size of each threads portion of the array */

		let func: fn(usize, usize, Arc<Vec<i64>>) -> (); /* reference to the function to run on the thread */
		match mode {
			1 => { func = mode1; }
			2 => {func = mode2; }
			3 => {func = mode3; }
			_ => { println!("invalid mode provided. See help for more details."); return;}
		}

		// spawn the threads with appropriate arguments
		for tidx in 0..threads_count {

			// Arguments which are passed to the thread
			let ref_clone = array_ref.clone();
			let start_idx = tidx * slice_size;
			let mut slice_size_thread = slice_size;
			if tidx == threads_count - 1 {
				slice_size_thread += array_size % threads_count;
			}

			// To-Do 1: Spawn a thread that calls 'func' and passes in the appropriate
			// arguments. Add the created thread to the 'threads' vector defined above.
			
		}

		// Wait for threads to complete
		for t in threads {
			//To-Do 2: Wait for threads to finish their jobs
		}
	}

	// Retrieve result based on mode used
	// (Different modes store results in different ways)
	let result: i64;
	match mode {
		0 => unsafe { result = GRAND_SUM }
		1 => unsafe { result = GRAND_SUM }
		2 => { result = *GRAND_SUM_WITH_LOCK.lock().unwrap() }
		3 => { result = *GRAND_SUM_WITH_LOCK.lock().unwrap() }
		_ => { println!("invalid mode provided. See help for more details."); return;}
	}

	// Print results
	let elapsed = now.elapsed();
	println!("\n# RESULT #");
	println!("Time elapsed: {}.{:06}", elapsed.as_secs(), elapsed.subsec_micros());
	println!("Grand Sum:{}", result);
}

