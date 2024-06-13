use std::collections::HashMap;

#[derive(Debug)]
enum MyNum {
    F(f32),
    I(i32),
}

fn main() {
    let mut v = vec![1, 6, 6, 3, 7, 9, 2];
    println!("Mean: {}", mean(&v));
    let med = median(&mut (v));
    match med {
        MyNum::F(f) => println!("Median: {f}"),
        MyNum::I(i) => println!("Median: {i}"),
    }
    let my_mode = mode(&v);
    match my_mode {
        Ok(n) => println!("Mode: {}", n),
        Err(e) => println!("{}", e),
    }
}

fn mean(v: &[i32]) -> f32 {
    let s: i32 = v.iter().sum();
    s as f32 / v.len() as f32
}

fn median(v: &mut [i32]) -> MyNum {
    v.sort();
    let l = v.len();
    if l % 2 != 0 {
        let i = l / 2;
        MyNum::I(v[i])
    } else {
        let i = l / 2;
        MyNum::F((v[i] + v[i - 1]) as f32 / 2.0)
    }
}

fn mode(v: &[i32]) -> Result<i32, String> {
    let mut freq = HashMap::new();
    for elem in v {
        let count = freq.entry(elem).or_insert(0);
        *count += 1;
    }
    let mut max = i32::MIN;
    for (_, v) in &freq {
        if *v > max {
            max = *v;
        }
    }

    let mut results: Vec<i32> = Vec::new();
    for (k, v) in &freq {
        if *v == max {
            results.push(**k);
        }
    }
    if results.len() == 1 {
        Ok(results[0])
    } else {
        Err(String::from("No mode found."))
    }
}
