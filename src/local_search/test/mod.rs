#![cfg(test)]

use std::marker::PhantomData;

use super::*;

struct MinimizeFn<F, Arg, Out>(PhantomData<(F, Arg, Out)>);

impl<F, Arg, Out> MinimizeFn<F, Arg, Out> {
   fn new() -> Self { MinimizeFn(PhantomData) }
}

impl<F, Arg, Out> Problem for MinimizeFn<F, Arg, Out>
   where F: for<'a> Fn(&'a Arg, ) -> Out,
         Out: PartialOrd
{
   type Instance = F;
   type Solution = Arg;
   type Cost = Out;

   fn check(_: &<Self as Problem>::Instance, _: &<Self as Problem>::Solution) -> bool { true }

   fn eval(f: &F, solution: &<Self as Problem>::Solution) -> <Self as Problem>::Cost { (f)(solution) }
}


struct Range<P>(u32, PhantomData<P>);

impl<P> Range<P> { fn new(r: u32) -> Self { Range(r, PhantomData) } }

//existential type MyIter: Iterator<Item=i32>;

impl<P> Neighborhood for Range<P>
   where P: Problem<Solution=i32>,
{
   type P = P;

   fn find<'i, F>(&self, mut current: Solution<'i, <Self as Neighborhood>::P>, mut predicate: F) -> MaybeNeighbor<'i, <Self as Neighborhood>::P>
                  where F: for<'a> FnMut(&'a Solution<Self::P>) -> bool {
      let s = *current.solution();
      let size = self.0 as i32;
      for i in (s - size..s + size + 1) {
         current.transform(|s| *s = i);
         if predicate(&current) {
            return MaybeNeighbor::Found(current)
         }
      }
      current.transform(|i| *i = s);
      MaybeNeighbor::NotFound(current)
   }

//
}

//   type Iter = MyIter;
//   fn iterator(&self, s: Solution<P>) -> <Self as Neighborhood>::Iter {
//      let s = s.solution();
//      let size = self.0 as i32;
//      (s - size..s + size + 1)
//   }
//}

#[test]
fn test_simple_convex() {
//   let initial = Const::<i32, MinimizeFn<_, _, _>>::new(0);
   let initial = 0i32;
   let neighborhood = Range::new(5);
   let hc = HillClimbing { initial, neighborhood, };
   let instance = |x: &i32| *(x.max(&-200));
   let solution: Solution<MinimizeFn<_, i32, i32>> = hc.solve(&instance, |s| println!("found new optimum: {} (cost: {})", s.solution(), s.cost()));
   let solution: i32 = *solution.solution();
   assert_eq!(solution, -200);
}


#[test]
fn test_simple_convex_float() {
   let hc = HillClimbing {
//      initial: Const::<_, MinimizeFn<_, _, _>>::new(0),
      initial: 0i32,
      neighborhood: Range::new(5),
   };
   let instance = |x: &i32| ((*x as f32) * 0.5).max(-200.0);
   let solution: Solution<MinimizeFn<_, i32, _>> = hc.solve(&instance, |s| println!("found new optimum: {} (cost: {})", s.solution(), s.cost()));
   let solution = *solution.solution();
   assert_eq!(solution, -400);
}

mod graph_coloring;
