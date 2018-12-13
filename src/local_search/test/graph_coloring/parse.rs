use super::dto::*;
use std::{
   io,
   io::{BufReader, BufRead},
   fs::File,
   path::Path
};

/// Reads the graph from a given file path.
pub fn read_graph(path: impl AsRef<Path>) -> io::Result<Graph> {
   let mut reader = BufReader::new(File::open(path)?).lines();
   let mut lines = (&mut reader)
      .take_while(|x| x.is_ok())
      .map(|x| x.unwrap());
   let _meta_data: Vec<_> = take_lines(&mut lines, "c").collect();
   let counts = take_lines(&mut lines, "p")
      .next()
      .and_then(|line| {
         let mut numbers = line.split_whitespace()
                               .map(|w| w.parse());
         match (numbers.next(), numbers.next()) {
            (Some(Ok(_node_cnt)), Some(Ok(edge_cnt))) => Some(edge_cnt),
            err => {
               eprintln!("unexpected format for edge and node count. parser returned: {:?}", err);
               None
            }
         }
      });
   let mut vec = match counts {
      Some(edge_cnt) => Vec::with_capacity(edge_cnt),
      None => Vec::new(),
   };
   let mut node_cnt = 0;
   for line in take_lines(&mut lines, "e") {
      let mut nums = line.split_whitespace()
                         .map(|w| w.parse());
      match (nums.next(), nums.next()) {
         (Some(Ok(v)), Some(Ok(w))) => {
            node_cnt = node_cnt.max(v).max(w);
            vec.push((Node(v - 1), Node(w - 1)));
         },
         _err => {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("unexpected format: {}", line)))
         }
      };
   }

   if let Some(something) = reader.next() {
      let line = something?;
      Err(io::Error::new(io::ErrorKind::InvalidData, format!("could not parse line: {}", line)))
   } else {
      Ok(Graph::new(node_cnt as usize, vec))
   }
}

/// Takes all lines starting with `prefix` and removing all those prefixes from the lines
fn take_lines<'a, 'b: 'a>(iter: &'a mut impl Iterator<Item=String>, prefix: &'b str) -> impl Iterator<Item=String> + 'a {
   iter.take_while(move |s| s.starts_with(prefix))
       .map(move |mut line| line.split_off(prefix.len()))
}
