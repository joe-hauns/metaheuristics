use rand::Rng;
use rand::seq::SliceRandom;

use super::*;

use self::dto::*;

mod dto;
mod parse;

struct GraphColoring;

impl GraphColoring {
   fn __raw_greedy_coloring(instance: &<Self as Problem>::Instance, solution: &<Self as Problem>::Solution) -> (Vec<Option<Color>>, usize) {
      let mut assignment: Vec<Option<Color>> = vec![None; solution.len()];
      let mut color_cnt = 0;
      for n in solution.iter().cloned() {
         let free = (0..color_cnt).map(Color)
                                  .find(|c| instance.neighborhood(n).iter().cloned()
                                                    .all(|m| assignment[m.0] != Some(*c)));
         assignment[n.0] = Some(
            match free {
               Some(c) => c,
               None => {
                  let c = color_cnt;
                  color_cnt += 1;
                  Color(c)
               }
            });
      }

      (assignment, color_cnt)
   }
   pub fn greedy_coloring(instance: &<Self as Problem>::Instance, solution: &<Self as Problem>::Solution) -> VecColoring {
      let (assignment, color_cnt) = Self::__raw_greedy_coloring(instance, solution);
      VecColoring {
         assignment: assignment.into_iter()
                               .map(|x| x.unwrap())
                               .collect(),
         color_cnt,
      }
   }
}

//struct LN {
//
//}
//
//struct PartialGraphColoring {
//
//}
//
//impl<'a> PartialSolution for PartialGraphColoring {
//   type SubProblem = SubGraphColoring;
//
//   fn to_super(&self, sub_solution: &Solution<'a, <Self as PartialSolution<'a, P>>::SubProblem>) -> Solution<'a, P> {
//      unimplemented!()
//   }
//
//   fn sub_problem(&self) -> <<Self as PartialSolution<P>>::SubProblem as Problem>::Instance {
//      unimplemented!()
//   }
//}
//
//impl LargeNeighborhood for LN {
//   type Partial = PartialGraphColoring;
//
//   fn select<'c>(&self, current: Solution<'c, P>) -> <Self as LargeNeighborhood<P>>::Partial {
//      unimplemented!()
//   }
//}

struct SwapNodeNeighborhood;

impl<P, T> Neighborhood<P> for SwapNodeNeighborhood
   where P: Problem<Solution=Vec<T>>,
         <P as Problem>::Cost: Clone,
{
   fn find<'i, F>(&self, mut current: Solution<'i, P>, mut predicate: F) -> MaybeNeighbor<'i, P>
                  where F: for<'a> FnMut(&'a Solution<P>) -> bool {
      let mut rand = rand::thread_rng();
      let cost = current.cost().clone();
      for _ in 0..current.solution().len() {
         let i = rand.gen_range(0, current.solution().len());
         let j = rand.gen_range(0, current.solution().len());
         current.transform(|s| s.swap(i, j));
         if predicate(&current) {
            return MaybeNeighbor::Found(current)
         }
         current.raw_transform(|s, c| {
            s.swap(i, j);
            *c = cost.clone();
         });
      }

      MaybeNeighbor::NotFound(current)
   }
}

impl Problem for GraphColoring {
   type Instance = Graph;
   type Solution = Vec<Node>;
   type Cost = usize;

   fn check(instance: &<Self as Problem>::Instance, solution: &<Self as Problem>::Solution) -> bool {
      true
   }

   fn eval(instance: &<Self as Problem>::Instance, solution: &<Self as Problem>::Solution) -> <Self as Problem>::Cost {
      Self::__raw_greedy_coloring(instance, solution).1
   }
}

struct Greedy;

impl InitialSolution<GraphColoring> for Greedy {
   fn get(&self, instance: &<GraphColoring as Problem>::Instance) -> <GraphColoring as Problem>::Solution {
      let mut rand = rand::thread_rng();
      let mut nodes: Vec<_> = instance.nodes()
                                      .collect();
      nodes.shuffle(&mut rand);
      nodes
   }
}

fn algorithm() -> impl Algorithm<GraphColoring> {
   SimulatedAnnealing {
      initial: Greedy,
      neighborhood: SwapNodeNeighborhood,
      create_random: || rand::thread_rng(),
      heat: DefaultTemperature {
         initial_temperature: 1000.0,
         cooling_factor: 1.05,
      }
   }
}


macro_rules! test {
    ($(file: $file:ident, $clsr:expr; )*) => {
    $(
      #[test]
      fn $file() {
         let graph = parse::read_graph(concat!("resources/", stringify!($file), ".col") ).unwrap();
         let algo = algorithm();
         let solution = algo.solve(&graph, |s| println!("current best: {}", s.cost()));
//         assert_eq!(*solution.cost(), $val);
         assert!($clsr(*solution.cost()), "expected: {}\ngot: {:?}", stringify!($clsr), solution.cost());
      }
      )*
    };
}

test! {
   file: myciel2    , |c| c ==  2;
   file: myciel3    , |c| c ==  3;
   file: myciel4    , |c| c ==  4;
   file: myciel5    , |c| 5 <= c && c <= 6;
   file: queen5_5   , |c| c ==  5;
   file: queen14_14 , |c| 18 <= c && c <= 19;
   file: jean       , |c| c == 10;
}
