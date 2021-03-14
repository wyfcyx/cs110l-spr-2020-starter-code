use crossbeam_channel;
use std::{thread, time};

fn parallel_map<T, U, F>(mut input_vec: Vec<T>, num_threads: usize, f: F) -> Vec<U>
where
    F: FnOnce(T) -> U + Send + Copy + 'static,
    T: Send + 'static,
    U: Send + 'static + Default,
{
    let mut output_vec: Vec<U> = Vec::with_capacity(input_vec.len());
    // TODO: implement parallel map!

    // channel: work dispatcher, main -> worker
    let (dispatch_s, dispatch_r) = crossbeam_channel::unbounded::<(usize, T)>();
    // channel: collect results, worker -> main
    let (collector_s, collector_r) = crossbeam_channel::unbounded::<(usize, U)>();
    // create worker threads
    let mut threads = Vec::new();
    for _ in 0..num_threads {
        let w_dispatch_r = dispatch_r.clone();
        let w_collector_s = collector_s.clone();
        threads.push(thread::spawn(move || {
            while let Ok((idx, input)) = w_dispatch_r.recv() {
                w_collector_s.send((idx, f(input))).unwrap();
            }
            drop(w_collector_s);
        }));
    }
    // dispatch inputs
    while let Some(input) = input_vec.pop() {
        dispatch_s.send((input_vec.len(), input)).unwrap();
    }
    drop(dispatch_s);
    // wait for all worker threads
    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
    // collect results
    for _ in 0..output_vec.capacity() {
        output_vec.push(U::default());
    }
    drop(collector_s);
    while let Ok((idx, result)) = collector_r.recv() {
        output_vec[idx] = result;
    }

    output_vec
}

fn main() {
    let v = vec![6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 12, 18, 11, 5, 20];
    let squares = parallel_map(v, 10, |num| {
        println!("{} squared is {}", num, num * num);
        thread::sleep(time::Duration::from_millis(500));
        num * num
    });
    println!("squares: {:?}", squares);
}
