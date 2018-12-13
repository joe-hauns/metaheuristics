use crate::problem::*;

pub use self::{
   hill_climbing::*,
   simulated_annealing::*,
};

mod hill_climbing;
mod simulated_annealing;

pub trait InitialSolution<P>
   where P: Problem
{
   fn get(&self, instance: &<P as Problem>::Instance) -> <P as Problem>::Solution;
}

pub trait Neighborhood {
   type P: Problem;
   type Iter: Iterator<Item=<Self::P as Problem>::Solution>;
   fn iterator<'i>(&self, current: Solution<Self::P>) -> Self::Iter;
}

//pub trait Neighborhood<'i>
//   where <Self::P as Problem>::Instance: 'i
//{
//   type P: Problem;
//   type Iter: Iterator<Item=Solution<'i, Self::P>>;
//   fn iterator(&self, current: Solution<'i, Self::P>) -> Self::Iter;
////   fn solution_iter(&self, current: Solution<Self::P>) -> Self::SolutionIter {
////      self.iterator(instance, solution)
////          .map(move |n| Solution::new(instance, n))
////   }
//   //   fn evaluated_iterator(&self, instance: &<Self::P as Problem>::Instance, solution: <Self::P as Problem>::Solution) -> Self::Iter2 {
////      self.iterator(instance, solution)
////          .map(move |n| Solution::new(instance, n))
////   }
//}

impl<'a, N> Neighborhood for &'a N
   where N: Neighborhood,
{

   type P = N::P;
   type Iter = N::Iter;

   fn iterator<'i>(&self, current: Solution<'i, <Self as Neighborhood>::P>) -> <Self as Neighborhood>::Iter {
      (*self).iterator(current)
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
