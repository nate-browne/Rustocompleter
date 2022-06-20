use std::fs::File;
use std::io::{BufRead, BufReader};

mod mwt;
use mwt::{Mwt, MwtNode};

const MIN_LEN: usize = 1;
const ELEMENTS_TO_RETURN: usize = 10;

/// First element is the rank, second is the word
type RetTup = (i32, String);

pub struct Autocompleter {
    trie: Mwt,
}

impl Autocompleter {
    pub fn new() -> Autocompleter {
        Autocompleter { trie: Mwt::new() }
    }

    pub fn from_dict(dict_filename: &String) -> Result<Autocompleter, String> {
        let mut val = Autocompleter::new();

        let dict_file = match File::open(dict_filename) {
            Ok(f) => f,
            Err(e) => return Err(format!("Error opening file: {e}")),
        };

        let reader = BufReader::new(dict_file);

        for line in reader.lines() {
            match line {
                Ok(l) => val.trie.add_record(l),
                Err(e) => return Err(format!("Error reading line from file: {e}")),
            }
        }

        Ok(val)
    }

    pub fn predict_completions(&self, prefix: &String) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut tmp = self.trie.get_root();

        if prefix.len() >= MIN_LEN {
            // Walk down the Trie as far as we can
            for ch in prefix.chars() {
                let children = tmp.get_children();

                if !children.contains_key(&ch) {
                    return res;
                }
                tmp = match children.get(&ch).unwrap() {
                    Some(nd) => nd,
                    None => panic!("Unreachable code hit: existing child had non-existing node!"),
                }
            }
            // Run DFS to get all completion predictions
            let mut dfs_results = Autocompleter::depth_first_search(Some(tmp));

            // Sort by alphabetical order first, then stable sort on frequency second
            dfs_results.sort_unstable_by(|a, b| a.1.cmp(&b.1));
            dfs_results.sort_by(|a, b| a.0.cmp(&b.0));

            let num_to_ret = if dfs_results.len() < ELEMENTS_TO_RETURN {
                dfs_results.len()
            } else {
                ELEMENTS_TO_RETURN
            };

            for ind in 0..num_to_ret {
                res.push(dfs_results[ind].1.clone());
            }
        }
        res
    }

    fn depth_first_search(node: Option<&Box<MwtNode>>) -> Vec<RetTup> {
        let mut ret: Vec<RetTup> = Vec::new();
        if let Some(nd) = node {
            let children = nd.get_children();

            if nd.get_end() {
                ret.push((nd.get_rank(), nd.get_data().to_string()));
            }

            for (_, value) in children.iter() {
                let recursive_res = Autocompleter::depth_first_search(value.as_ref());
                if !recursive_res.is_empty() {
                    for item in recursive_res {
                        ret.push(item);
                    }
                }
            }
        }
        ret
    }
}
