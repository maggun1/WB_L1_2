use std::thread;

fn main() {
    let n: usize = 10;              // Количество чисел
    let num_threads: usize = 5;     // Количество потоков (<=n/2)
    let numbers: Vec<usize> = (1..=n).collect();

    // Количество чисел, которые обработает каждый поток
    let chunk_size: usize = (n as f32 / num_threads as f32).ceil() as usize;

    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, n);

        let chunk = numbers[start..end].to_vec();

        let handle = thread::spawn(move || {
            for num in chunk {
                println!("{:?};\t Number: {};\t Square: {};", thread::current().id(), num, num * num);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
