use std::collections::HashMap;
use std::collections::VecDeque;

use super::parse::get_and_parse_html;
use std::result::Result;

#[derive(Debug, PartialEq)]
pub struct ParseTree {
    pub root: HTMLContent,
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub tag_type: String,
    pub attributes: HashMap<String, String>,
    pub content: Vec<HTMLContent>,
}

#[derive(Debug, PartialEq)]
pub enum HTMLContent {
    Raw(String),
    Tag(Box<Tag>),
}

// Maybe have some sort of string condition format to parse
// is PreOrder by default?
// e.g. (tag=a, attrs=href,align)
// (tag=a|h1)
// ((tag=a & attr_val=(align,center)) | tag=h1)
//
// tree.search(Pre or Level, condition string) calls Searcher constructor which returns iterator?
// OR Searcher::new(Pre or Level, condition string) and searcher1(tree)
// OR BOTH!!
pub struct Searcher {
    // probably just create the closure upon creation, no need to store values...
//    is_pre_ord: bool,
//    tag_types: Vec<&'a str>,
//    has_attrs: Vec<&'a str>,
//    attr_values: Vec<(&'a str,&'a str)>,
}

pub struct PtPreOrderIter<'a> {
    stack: Vec<&'a HTMLContent>,
}

pub struct PtLevelOrderIter<'a> {
    queue: VecDeque<&'a HTMLContent>,
}

pub struct TreeIter<'a> {
    is_pre_order: bool,
    state: VecDeque<&'a HTMLContent>,
}

impl Tag {
    pub fn get_tag(&self) -> &String {
        &self.tag_type
    }
}

impl ParseTree {

    pub fn new(url: &str) -> Self {
        get_and_parse_html(url).unwrap()
    }

    pub fn pre_iter(&self) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(true,&self.root)
    }

    pub fn level_iter(&self) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(false,&self.root)
    }

    pub fn iter(&self, is_pre_order: bool) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(is_pre_order, &self.root)
    }

    pub fn search(&self, is_pre_order: bool, search_pattern: &str) -> Result<impl Iterator<Item = &HTMLContent>, &'static str> {
        match Searcher::generate_closure(search_pattern) {
            Err(e) => Err(e),
            Ok(c) => Ok(self.iter(is_pre_order).filter(c))
        }
    }

}

// duplicate code from tree, is it worth distinguishing???
impl HTMLContent {
    pub fn pre_iter(&self) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(true,self)
    }

    pub fn level_iter(&self) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(false,self)
    }

    pub fn iter(&self, is_pre_order: bool) -> impl Iterator<Item = &HTMLContent> {
        TreeIter::new(is_pre_order, self)
    }

    pub fn search(&self, is_pre_order: bool, search_pattern: &str) -> Result<impl Iterator<Item = &HTMLContent>, &'static str> {
        match Searcher::generate_closure(search_pattern) {
            Err(e) => Err(e),
            Ok(c) => Ok(self.iter(is_pre_order).filter(c))
        }
    }
}

impl Searcher {

    fn generate_closure(pattern: &str) -> Result<impl Fn(&&HTMLContent) -> bool,&'static str> {
        if pattern == "raw" {
            let foo = |node: &&HTMLContent| match **node {
                HTMLContent::Tag(_) => false,
                HTMLContent::Raw(_) => true
            };
            Ok(foo)
        }
        else {
            Err("could not parse search pattern")
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = &'a HTMLContent;
    fn next(&mut self) -> Option<&'a HTMLContent> {
        if let Some(node) = self.state.pop_front() {
            if let HTMLContent::Tag(tag_box) = node { //add children to stack if tag object
                if self.is_pre_order {
                    for child in (**tag_box).content.iter().rev() {
                        self.state.push_front(child);
                    }
                }
                else {
                    for child in (**tag_box).content.iter() {
                        self.state.push_back(child);
                    }
                }

            }
            return Some(node)
        }
        else {
            None
        }
    }
}

impl<'a> TreeIter<'a> {
    pub fn new(is_pre_order: bool, content: &'a HTMLContent) -> Self {
        let mut temp_queue = VecDeque::new();
        temp_queue.push_back(content);
        TreeIter {
            is_pre_order,
            state: temp_queue,
        }
    }
}

impl ParseTree {

    pub fn testing_tree() -> Self {

        let h1 = Tag {
            tag_type: "h1".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("foo".to_owned())]
        };

        let a1 = Tag {
            tag_type: "a".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("link".to_owned())]
        };

        let div1 = Tag {
            tag_type: "div".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(h1)), HTMLContent::Tag(Box::new(a1))]
        };

        let a2 = Tag {
            tag_type: "a".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("bar".to_owned())]
        };

        let p = Tag {
            tag_type: "p".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("baz".to_owned()), HTMLContent::Tag(Box::new(a2)), HTMLContent::Raw("qux".to_owned())]
        };

        let div2 = Tag {
            tag_type: "div".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(p))]
        };

        let body = Tag {
            tag_type: "body".to_owned(),
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(div1)),HTMLContent::Tag(Box::new(div2))]
        };

        ParseTree {
            root: HTMLContent::Tag(Box::new(body))
        }

    }
}