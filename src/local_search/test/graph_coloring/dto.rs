use std::collections::BTreeSet;
use std::ops::{Add, Rem, Sub};

use std::ops::IndexMut;
use std::ops::Index;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash, Ord, PartialOrd)]
pub struct Color(pub usize);

#[derive(Eq, PartialOrd, PartialEq, Ord, Debug, Clone, Copy, Hash)]
pub struct Node(pub usize);

macro_rules! num_ops {
    ($new_type:ident, $number:ident) => {

      impl Add<$number> for $new_type {
         type Output = $new_type ;

         fn add(self, rhs: $number) -> $new_type  {
            $new_type (self.0 + rhs)
         }
      }
      impl Sub<$number> for $new_type {
         type Output = $new_type ;

         fn sub(self, rhs: $number) -> $new_type  {
            $new_type (self.0 - rhs)
         }
      }

      impl Rem<$number> for $new_type  {
         type Output = $new_type ;

         fn rem(self, rhs: $number) -> $new_type  {
            $new_type (self.0 % rhs)
         }
      }

      impl Into<$number> for $new_type  {
         fn into(self) -> $number  {
            self.0
         }
      }
//      impl Into<$number> for $new_type  {
//         fn into(self) -> $number  {
//            self.0 as _
//         }
//      }

    };
}

num_ops!(Color, usize);
num_ops!(Node, usize);

#[derive(Debug)]
pub struct Graph {
   list: Vec<BTreeSet<Node>>,
}

impl Graph {
   pub fn nodes(&self) -> impl Iterator<Item=Node> {
      (0..self.list.len()).map(Node)
   }
   pub fn neighborhood(&self, node: Node) -> &BTreeSet<Node> {
      &self.list[node.0]
   }
   pub fn neighbors<'a>(&'a self, node: Node) -> impl Iterator<Item=Node> + 'a {
      self.list[node.0].iter().cloned()
   }

   pub fn new(node_cnt: usize, edges: Vec<(Node, Node)>) -> Self {
      let mut list = vec![BTreeSet::new(); node_cnt];
      for (v, w) in edges {
         if v == w {
            eprintln!("invalid graph input: has self loops. Skipping {:?}", (v, w));
         } else {
            list[v.0].insert(w);
            list[w.0].insert(v);
         }
      }

      let g = Graph { list, };
      assert!(g.nodes()
               .all(|n| {
                  g.neighbors(n)
                   .all(|m| g.neighbors(m).any(|n1| n1 == n))
               }));
      g.nodes()
       .for_each(|n| g.neighbors(n)
                      .for_each(|m| assert_ne!(m, n)));
      g
   }

   pub fn edge(&self, v: Node, w: Node) -> bool {
      self.list[v.0].contains(&w)
   }

   pub fn node_cnt(&self) -> usize {
      self.list.len() as _
   }

   fn forall_neighbors<P>(&self, of: Node, predicate: P) -> bool
                          where P: FnMut(Node) -> bool {
      self.list[of.0].iter()
                     .cloned()
                     .all(predicate)
   }
}

#[derive(Debug, Clone)]
pub struct VecColoring {
   pub assignment: Vec<Color>,
   pub color_cnt: usize,
}

impl Index<Node> for VecColoring {
   type Output = Color;

   fn index(&self, idx: Node) -> &Color {
      let idx: usize = idx.into();
      &self.assignment[idx]
   }
}

impl IndexMut<Node> for VecColoring {
   fn index_mut(&mut self, index: Node) -> &mut Color {
      &mut self.assignment[index.0]
   }
}

impl VecColoring {
   pub fn color_cnt(&self) -> usize {
      self.color_cnt
   }
   pub fn colors<'a>(&'a self) -> impl Iterator<Item=Color> + 'a {
      self.assignment
          .iter()
          .cloned()
   }

   #[allow(unused)]
   pub fn iter<'a>(&'a self) -> impl Iterator<Item=(Node, Color)> + 'a {
      self.assignment
          .iter()
          .enumerate()
          .map(|(i, &c)| (Node(i), c))
   }
}

