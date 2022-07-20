pub mod wrap;
pub use wrap::*;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn compute_fibonacci(args: ArgsComputeFibonacci) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    for n in 0..args.n {
        result.push(fibonacci(n));
    }

    result
}
