use crate::problem::*;

pub use self::{
   hill_climbing::*,
   large_neighborhood_search::*,
   simulated_annealing::*,
};

pub mod hill_climbing;
pub mod simulated_annealing;
pub mod large_neighborhood_search;

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

pub trait Neighborhood<P: Problem> {
   //   type P: Problem;
   fn find<'i, F>(&self, current: Solution<'i, P>, predicate: F) -> MaybeNeighbor<'i, P>
                  where F: for<'a> FnMut(&'a Solution<P>) -> bool;
}

impl<'a, N, P> Neighborhood<P> for &'a N
   where P: Problem,
         N: Neighborhood<P>,
{
   fn find<'i, F>(&self, current: Solution<'i, P>, predicate: F) -> MaybeNeighbor<'i, P>
                  where F: for<'f> FnMut(&'f Solution<P>) -> bool
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
