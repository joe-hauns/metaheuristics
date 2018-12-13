use super::*;

pub struct LargeNeighborhoodSearch<I, L, A> {
   initial: I,
   neighborhood: L,
   algorithm: A,
}

pub trait PartialSolution<'i, P: Problem> {
   type SubProblem: Problem;
   fn to_complete(&self, sub_solution: &Solution<Self::SubProblem>) -> Solution<'i, P>;
   fn sub_problem(&self) -> <Self::SubProblem as Problem>::Instance;
}

pub trait LargeNeighborhood<'i, P: Problem> {
   type Partial: PartialSolution<'i, P>;
   fn select<'c: 'i>(&self, current: Solution<'i, P>) -> Self::Partial;
}

impl<P, I, L, A> Algorithm<P> for LargeNeighborhoodSearch<I, L, A>
   where P: Problem,
         I: InitialSolution<P>,
         L: for<'i> LargeNeighborhood<'i, P>,
         A: for<'a> Algorithm<<<L as LargeNeighborhood<'a, P>>::Partial as PartialSolution<'a, P>>::SubProblem>
{
   fn solve<'i, F>(&self, instance: &'i <P as Problem>::Instance, mut callback: F) -> Solution<'i, P>
                   where F: for<'a> FnMut(&'a Solution<P>) {
      let initial = Solution::new(instance, self.initial.get(instance));

      let partial_solution = self.neighborhood.select(initial);
      let sub_problem = partial_solution.sub_problem();
      let sub_solution = self.algorithm.solve(&sub_problem, |sub| callback(&partial_solution.to_complete(sub)));
      partial_solution.to_complete(&sub_solution)
   }
}