/// Root library file for slam-rs
pub mod base;
pub mod factors;
pub mod geometry;
pub mod graphs;
pub mod solvers;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
