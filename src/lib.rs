#![feature(existential_type, nll)]
//#![feature(existential_type, nll, unboxed_closures, impl_trait_in_bindings)]

mod problem;
mod local_search;

#[cfg(test)]
mod tests {
   use num::cast::AsPrimitive;
    #[test]
    fn it_works() {
       let x: f32 = 0usize.as_();
        assert_eq!(2 + 2, 4);
    }
}
