use std::cmp::Ordering;

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

struct SwapNodeNeighborhood;
//existential type SwapNodeIter: Iterator<Item=Vec<Node>>;

impl Neighborhood for SwapNodeNeighborhood {
   type P = GraphColoring;

   fn find<'i, F>(&self, mut current: Solution<'i, <Self as Neighborhood>::P>, mut predicate: F) -> MaybeNeighbor<'i, <Self as Neighborhood>::P>
                  where F: for<'a> FnMut(&'a Solution<Self::P>) -> bool {
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

//   type Iter = SwapNodeIter;
//   fn iterator<'i>(&self, mut current: Solution<'i, <Self as Neighborhood>::P>) -> <Self as Neighborhood>::Iter {
//      let mut rand = rand::thread_rng();
//      let mut current = current.destroy().0;
//      (0..current.len()).map(move |_| {
//         let i = rand.gen_range(0, current.len());
//         let j = rand.gen_range(0, current.len());
//         current.swap(i, j);
//         let out = current.clone();
//         current.swap(i, j);
//         out
//      })
//   }
}

impl Problem for GraphColoring {
   type Instance = Graph;
   type Solution = Vec<Node>;
   type Cost = usize;

   fn check(instance: &<Self as Problem>::Instance, solution: &<Self as Problem>::Solution) -> bool {
      true
//      instance.nodes()
//              .all(|v| instance.neighborhood(v).iter().cloned()
//                               .all(|w| solution[v] != solution[w]))
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

fn algorithm() -> impl Algorithm<P=GraphColoring> {
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
    ($(file: $file:ident, expected: $val:expr; )*) => {
    $(
      #[test]
      fn $file() {
         let graph = parse::read_graph(concat!("resources/", stringify!($file), ".col") ).unwrap();
         let algo = algorithm();
         let solution = algo.solve(&graph, |s| println!("current best: {}", s.cost()));
         assert_eq!(*solution.cost(), $val);
      }
      )*
    };
}

test! {
   file: myciel2    , expected: 2;
   file: myciel3    , expected: 3;
   file: myciel4    , expected: 4;
   file: myciel5    , expected: 5;
   file: queen5_5   , expected: 5;
   file: queen14_14 , expected: 18;
   file: jean       , expected: 10;
}
