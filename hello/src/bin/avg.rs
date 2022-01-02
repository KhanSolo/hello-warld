use rayon::prelude::*;

fn main() {
    let arr = [1., 2., 3., 4., 5., 6., 7., 8.];
    println!("{}", avg1(&arr));
    println!("{}", avg2(&arr));
    println!("{}", avg3(&arr));
    println!("{}", avg4(&arr));

    std::process::exit(0);
}

fn avg1(list: &[f64]) -> f64 {
    let mut total = 0.;

    for el in list {
        total += *el;
    }
    return total / list.len() as f64;
}

fn avg2(list: &[f64]) -> f64 {
    list.iter().sum::<f64>() / list.len() as f64
}

fn avg3(list: &[f64]) -> f64 {
    list.par_iter().sum::<f64>() / list.len() as f64
}

fn avg4(list: &[f64]) -> f64 {
    list.iter().fold(0., |a, b| a + b) / list.len() as f64
}
