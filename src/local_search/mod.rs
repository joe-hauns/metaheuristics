use crate::problem::*;

pub use self::{
   hill_climbing::*,
   simulated_annealing::*,
};

pub mod hill_climbing;
pub mod simulated_annealing;

pub trait InitialSolution<P>
   where P: Problem
{
   fn get(&self, instance: &<P as Problem>::Instance) -> <P as Problem>::Solution;
}

pub enum MaybeNeighbor<'a, P>
   where P: Problem {
   Found(Solution<'a, P>),
   NotFound(Solution<'a, P>),
}

pub trait Neighborhood {
   type P: Problem;
   fn find<'i, F>(&self, current: Solution<'i, Self::P>, predicate: F) -> MaybeNeighbor<'i, Self::P>
                  where F: for<'a> FnMut(&'a Solution<Self::P>) -> bool;
}

impl<'a, N> Neighborhood for &'a N
   where N: Neighborhood,
{
   type P = N::P;

   fn find<'i, F>(&self, current: Solution<'i, <Self as Neighborhood>::P>, predicate: F) -> MaybeNeighbor<'i, Self::P>
                  where F: for<'f> FnMut(&'f Solution<Self::P>) -> bool
   {
      (*self).find(current, predicate)
   }
}


impl<P> InitialSolution<P> for <P as Problem>::Solution
   where P: Problem,
         Self: Clone
{
   fn get(&self, _instance: &<P as Problem>::Instance) -> <P as Problem>::Solution {
      self.clone()
   }
}

mod test;
