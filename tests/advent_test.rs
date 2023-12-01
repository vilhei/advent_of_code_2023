#![feature(test)]


fn main() {
    
}




#[cfg(test)]
mod tests {
    use test::Bencher;

    extern crate test;
    #[bench]
    fn bench_task(b: &mut Bencher) {}
}
