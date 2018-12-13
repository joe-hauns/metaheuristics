use super::*;

pub struct HillClimbing<I, N> {
   pub initial: I,
   pub neighborhood: N,
}

impl<P, I, N> Algorithm<P> for HillClimbing<I, N>
   where P: Problem,
         I: InitialSolution<P>,
         N: Neighborhood<P>,
         <P as Problem>::Solution: Clone,
         <P as Problem>::Cost: Clone,
{
   fn solve<'i, F>(&self, instance: &'i <P as Problem>::Instance, mut callback: F) -> Solution<'i, P>
                   where F: for<'a> FnMut(&'a Solution<P>) {
      use self::MaybeNeighbor::*;
      let best = Solution::new(instance, self.initial.get(&instance));
      callback(&best);
      let mut best = Some(best);

      loop {
         let cost = best.as_ref().unwrap().cost().clone();
         match self.neighborhood.find(best.take().unwrap(), |n| n.cost() < &cost) {
            Found(s) => {
               callback(&s);
               best = Some(s);
            }
            NotFound(s) => return s
         }
      }
   }
}

