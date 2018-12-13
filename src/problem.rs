pub trait Problem {
   type Instance;
   type Solution;
   type Cost: PartialOrd;
   fn check(instance: &Self::Instance, solution: &Self::Solution) -> bool;
   fn eval(instance: &Self::Instance, solution: &Self::Solution) -> Self::Cost;
}

pub trait Algorithm<P>
   where P: Problem
{
   //   type P: Problem;
   fn solve<'i, F>(&self, instance: &'i <P as Problem>::Instance, callback: F) -> Solution<'i, P>
                   where F: for<'a> FnMut(&'a Solution<P>);
}


#[derive(Debug)]
pub struct Solution<'a, P>
   where P: Problem,
{
   instance: &'a P::Instance,
   solution: P::Solution,
   cost: P::Cost,
}

impl<'a, P> Clone for Solution<'a, P>
   where P: Problem,
         <P as Problem>::Solution: Clone,
         <P as Problem>::Cost: Clone,
{
   fn clone(&self) -> Self {
      Solution {
         instance: self.instance,
         solution: self.solution.clone(),
         cost: self.cost.clone(),
      }
   }
}

impl<'a, P> Solution<'a, P>
   where P: Problem
{
   pub fn new(instance: &'a P::Instance, solution: P::Solution) -> Self {
      Solution {
         instance,
         cost: P::eval(instance, &solution),
         solution,
      }
   }

   pub fn raw_transform<F>(&mut self, f: F)
                           where F: FnOnce(&mut P::Solution, &mut P::Cost)
   {
      f(&mut self.solution, &mut self.cost);
   }

   pub fn transform<F>(&mut self, f: F)
                       where F: FnOnce(&mut P::Solution)
   {
      f(&mut self.solution);
      self.cost = P::eval(self.instance, &self.solution);
   }

//   pub fn map<F>(self, f: F) -> Self
//                 where F: FnOnce(P::Solution) -> P::Solution
//   {
//      let Solution {
//         instance,
//         solution,
//         cost: _,
//      } = self;
//      Self::new(instance, f(solution))
//   }

   pub fn instance(&self) -> &P::Instance { &self.instance }
   pub fn solution(&self) -> &P::Solution { &self.solution }
   pub fn cost(&self) -> &P::Cost { &self.cost }
   pub fn destroy(self) -> (P::Solution, P::Cost) {
      let Solution {
         instance: _, cost, solution,
      } = self;
      (solution, cost)
   }
}
