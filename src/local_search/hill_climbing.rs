use super::*;

pub struct HillClimbing<I, N> {
   pub initial: I,
   pub neighborhood: N,
}

impl<I, N> Algorithm for HillClimbing<I, N>
   where I: InitialSolution<N::P>,
         N: Neighborhood,
         <N::P as Problem>::Solution: Clone,
         <N::P as Problem>::Cost: Clone,
{
   type P = N::P;

   fn solve<'i, F>(&self, instance: &'i <<Self as Algorithm>::P as Problem>::Instance, mut callback: F) -> Solution<'i, Self::P>
                   where F: for<'a> FnMut(&'a Solution<Self::P>) {
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


//impl<'n, I, N> Algorithm for HillClimbing<I, N>
//   where I: InitialSolution,
//         N: Neighborhood<'n, P=I::P>,
//         <I::P as Problem>::Instance: 'n,
//         <I::P as Problem>::Solution: Clone,
//         <I::P as Problem>::Cost: Clone,
//{
//   type P = I::P;
//
//   fn solve<'i, F>(&self, instance: &'i <<Self as Algorithm>::P as Problem>::Instance, mut callback: F) -> Solution<'i, Self::P>
//                   where F: for<'a> FnMut(&'a Solution<Self::P>) {
//      let mut best = Solution::new(instance, self.initial.get(&instance));
//
//      while let Some(s) = self.neighborhood.iterator(best.clone())
//                              .map(|n| Solution::new(instance, n))
//                              .find(|n| n.cost() < best.cost()) {
//         best = s;
//         callback(&best);
//      }
//
//      best
//   }
//}

