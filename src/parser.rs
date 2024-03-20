use ptree::Style;
use std::borrow::Cow;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TreeNodeQuery {
    content: String,
    left: Option<Box<TreeNodeQuery>>,
    right: Option<Box<TreeNodeQuery>>,
}

impl TreeNodeQuery {
    fn new(content: String) -> Self {
        TreeNodeQuery {
            content,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, child_node: TreeNodeQuery, direction: &str) {
        match direction {
            "left" => {
                if self.left.is_none() {
                    self.left = Some(Box::new(child_node))
                }
            }
            "right" => {
                if self.right.is_none() {
                    self.right = Some(Box::new(child_node))
                }
            }
            _ => (),
        }
    }

    fn insert_as_child(self, mut parent_node: TreeNodeQuery, direction: &str) -> TreeNodeQuery {
        match direction {
            "left" => {
                parent_node.left = Some(Box::new(self));
            }
            "right" => {
                parent_node.right = Some(Box::new(self));
            }

            _ => (),
        };
        parent_node
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    // Need to always clone, because pushing/inserting always takes ownership
    pub fn breadth_first_node(&self) -> Option<String> {
        let mut visited: HashSet<TreeNodeQuery> = HashSet::new();
        let mut queue: VecDeque<TreeNodeQuery> = VecDeque::new();

        visited.insert(self.clone());
        queue.push_back(self.clone());

        while let Some(current_node) = queue.pop_front() {
            if current_node.content != "+" && current_node.content != "*" {
                return Some(current_node.content);
            }

            if let Some(left_node) = current_node.left.as_deref() {
                if !visited.contains(left_node) {
                    visited.insert(left_node.clone());
                    queue.push_back(left_node.clone())
                }
            }
            if let Some(right_node) = current_node.right.as_deref() {
                if !visited.contains(right_node) {
                    visited.insert(right_node.clone());
                    queue.push_back(right_node.clone())
                }
            }
        }
        None
    }
}

//update tree logic
impl TreeNodeQuery {
    pub fn update_tree(&mut self, query: &str, found: bool) -> bool {
        let mut did_update = false;
        let can_update = match (&self.left, &self.right) {
            (Some(_), Some(_)) => true,
            _ => false,
        };

        if can_update {
            let should_update_once =
                if let (Some(ref left_box), Some(ref right_box)) = (&self.left, &self.right) {
                    &left_box.content[..] == query || &right_box.content[..] == query
                } else {
                    false
                };

            if should_update_once {
                self.update_atomic(query, found);
                did_update = true;
            } else {
                did_update |= self.left.as_mut().unwrap().update_tree(query, found);
                did_update |= self.right.as_mut().unwrap().update_tree(query, found);
            }
        } else {
            return false;
        }

        did_update
    }
    fn update_atomic(&mut self, term: &str, found: bool) {
        if self.left.as_ref().unwrap().content == term {
            match &self.content[..] {
                "+" => {
                    *self = if found {
                        *self.left.take().unwrap()
                    } else {
                        *self.right.take().unwrap()
                    };
                }
                "*" => {
                    *self = if found {
                        *self.right.take().unwrap()
                    } else {
                        *self.left.take().unwrap()
                    };
                }
                _ => (),
            }
        } else {
            match &self.content[..] {
                "+" => {
                    *self = if found {
                        *self.right.take().unwrap()
                    } else {
                        *self.left.take().unwrap()
                    };
                }
                "*" => {
                    *self = if found {
                        *self.left.take().unwrap()
                    } else {
                        *self.right.take().unwrap()
                    };
                }
                _ => (),
            }
        };
    }
}

//display the tree using ptree crate
impl ptree::TreeItem for TreeNodeQuery {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        write!(f, "{}", self.content)
    }
    fn children(&self) -> Cow<[Self::Child]> {
        let mut children_vec: Vec<TreeNodeQuery> = Vec::new();

        if let Some(ref left_child) = self.left {
            children_vec.push(*left_child.clone());
        }

        if let Some(ref right_child) = self.right {
            children_vec.push(*right_child.clone());
        }

        Cow::from(children_vec)
    }
}

// Skips if we are pushing whitespace before or after adding +, *, (, ), final token
fn trim_whitespace(tokens: &mut VecDeque<TreeNodeQuery>, current_token: &mut String) {
    if !current_token.trim().is_empty() {
        tokens.push_back(TreeNodeQuery::new(current_token.trim().to_string()));
        current_token.clear();
    }
}

fn tokenize(args: &str) -> VecDeque<TreeNodeQuery> {
    let mut output: VecDeque<TreeNodeQuery> = VecDeque::new();
    let mut current_token = String::new();

    for current_character in args.chars() {
        match current_character {
            '+' | '*' => {
                if !current_token.is_empty() {
                    trim_whitespace(&mut output, &mut current_token);
                }
                output.push_back(TreeNodeQuery::new(current_character.to_string()));
            }
            '(' | ')' => {
                if !current_token.is_empty() {
                    trim_whitespace(&mut output, &mut current_token);
                }
                output.push_back(TreeNodeQuery::new(current_character.to_string()));
            }
            _ => {
                current_token.push(current_character);
            }
        }
    }
    if !current_token.is_empty() {
        trim_whitespace(&mut output, &mut current_token);
    }
    output
}

//parser logic
fn parser_or(tokens: &mut VecDeque<TreeNodeQuery>) -> TreeNodeQuery {
    let mut root = parser_and(tokens);

    while tokens
        .get_mut(0)
        .unwrap_or(&mut TreeNodeQuery::new(String::from("EOF")))
        .content
        == '+'.to_string()
    {
        let new_root = tokens.pop_front().unwrap();
        root = root.insert_as_child(new_root, "left");
        let new_right_child = parser_and(tokens);
        root.insert(new_right_child, "right");
    }

    root
}

fn parser_and(tokens: &mut VecDeque<TreeNodeQuery>) -> TreeNodeQuery {
    let mut root = parser_atomic(tokens);
    while tokens
        .get_mut(0)
        .unwrap_or(&mut TreeNodeQuery::new(String::from("EOF")))
        .content
        == '*'.to_string()
    {
        let new_root = tokens.pop_front().unwrap();
        root = root.insert_as_child(new_root, "left");
        let new_right_child = parser_atomic(tokens);
        root.insert(new_right_child, "right");
    }
    root
}

fn parser_atomic(tokens: &mut VecDeque<TreeNodeQuery>) -> TreeNodeQuery {
    let default = TreeNodeQuery::new(String::from("EOF"));

    if let Some(node) = tokens.pop_front() {
        match &node.content[..] {
            "(" => {
                let expression = parser_or(tokens);
                tokens.pop_front();
                return expression;
            }
            _ => {
                return node;
            }
        }
    };

    default
}

pub fn parse_query(args: &str) -> TreeNodeQuery {
    let mut tokens = tokenize(args);

    let parsed_tree = parser_or(&mut tokens);
    parsed_tree
}
#[cfg(test)]

mod parser_tests {
    use crate::parser::*;

    #[test]
    fn does_tokenize() {
        let test_string = String::from(" (Word 1 + word2 )  * and maybe *   (  this is + asds ) ");
        let test_list = tokenize(&test_string);
        let test_list_string: Vec<String> = test_list.into_iter().map(|x| x.content).collect();
        assert_eq!(
            vec![
                "(",
                "Word 1",
                "+",
                "word2",
                ")",
                "*",
                "and maybe",
                "*",
                "(",
                "this is",
                "+",
                "asds",
                ")"
            ],
            test_list_string
        );
    }

    #[test]
    fn update_tree_not_found() {
        let test_string = String::from(" (A + B) * ( C+F) *  (D  + E)  ");
        let mut test_list = tokenize(&test_string);
        let mut tree = parser_or(&mut test_list);
        println!("\n Tree before updating (A is not found):");
        ptree::print_tree(&tree).unwrap();
        while tree.update_tree("A", false) {}
        println!("\n Tree after updating:");
        ptree::print_tree(&tree).unwrap();
    }

    #[test]
    fn print_tree() {
        let test_string = String::from(" ((A + B) + ( A*C)) +  ((A+C) *B)  ");
        let mut test_list = tokenize(&test_string);
        let mut tree = parser_or(&mut test_list);
        ptree::print_tree(&tree).unwrap();

        #[test]
        fn update_tree_found() {
            let test_string = String::from(" (A + B) +( C * E)  ");
            let mut test_list = tokenize(&test_string);
            let mut tree = parser_or(&mut test_list);
            println!("\n Tree before updating (A is found):");
            ptree::print_tree(&tree).unwrap();
            while tree.update_tree("A", true) {}
            println!("\n Tree after updating:");
            ptree::print_tree(&tree).unwrap();
        }

        #[test]
        fn find_first_leaf() {
            let test_string = String::from(" (A + B) *( C+F) *  (D  + E) ");
            let mut test_list = tokenize(&test_string);
            let tree = parser_or(&mut test_list);
            print!("\n");
            let first_node = tree.breadth_first_node().unwrap();
            assert_eq!(first_node, "D");
        }
    }
}
