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

pub struct TreeIter<'a> {
    is_pre_order: bool,
    state: VecDeque<&'a HTMLContent>,
}

// If implemented this iter could allow for pruned search...
// It probably has to be its own type, separate from tree iter if we want caching with TreeIter
pub struct SearchTreeIter<'a, F: Fn(&HTMLContent) -> bool> {
    is_pre_order: bool,
    prune_closure: F,
    state: VecDeque<&'a HTMLContent>,
}

// Potential implementation of cached iters over certain size threshold, maybe thresh is if created a root
// Implement drop bool? and on drop trait?? to cache instead of drop when scope is exited
// I dont think you can cache search iters though... because they have specific closure type on creation
// potentially instead of start at root, way to check how much space has already been allocated to iter
// might actually be optimal to always cache, ask Tov...

// Who would even own this? mutable statics are unsafe so probably best to avoid
// Iters might be able to refer to the cache in a struct variable? with RC and RefCell?
// Could belong to the tree but we don't really want a separate cache for every HTML doc, is that the only way?

// maybe have some sort of wrapper data structure to produce trees and such from
// must produce trees from this func and holds the cache?
// let bsr = BSRObject::new() // intialize
// let tree = bsr.build_tree("url")
// ... same subsequent interactions
pub struct IterCache<'a> { // lifetimes may make it weird
// maybe use boxes to prevent copying around? is that necessary?
    // might not be able to use boxes, they indicate ownership right??? if so i think we should use them
    cache: Vec<TreeIter<'a>>,
}

//impl<'a> Drop for TreeIter<'a> {
//    fn drop(&mut self) {
//
//    }
//}


pub struct BSRObject<'a> {
    cache: Vec<TreeIter<'a>>,
}

impl<'a> BSRObject<'a> {
    pub fn new() -> Self {
        BSRObject {
            cache: Vec::new(),
        }
    }
    pub fn build_tree(&self, url: &str) -> ParseTree {
        ParseTree::new(url) //pass it cache ref?
    }
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

    // pruned search temp, should do it generically with similar pattern to example on no prune
    pub fn search2(&self, is_pre_order: bool, search_pattern: &str) -> Result<impl Iterator<Item = &HTMLContent>, &'static str> {
        match Searcher::generate_closure2(search_pattern) {
            Err(e) => Err(e),
            Ok((keep,prune)) => Ok(SearchTreeIter::new(is_pre_order,prune, &self.root).filter(keep))
        }
    }

}

// duplicate code from tree, is it worth distinguishing, maybe only have HTMLContent and no additional tree???
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
        if pattern == "raw" { // temp pattern...
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

    fn generate_closure2(pattern: &str) -> Result<(impl Fn(&&HTMLContent) -> bool, impl Fn(&HTMLContent) -> bool),&'static str> {
        if pattern == "raw" { // temp pattern...
            let foo = |node: &&HTMLContent| match **node {
                HTMLContent::Tag(_) => false,
                HTMLContent::Raw(_) => true
            };
            let bar = |node: &HTMLContent| match *node {
                HTMLContent::Tag(_) => false,
                HTMLContent::Raw(_) => false
            };
            Ok((foo,bar))
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

impl<'a, F: Fn(&HTMLContent) -> bool> Iterator for SearchTreeIter<'a, F> {
    type Item = &'a HTMLContent;
    fn next(&mut self) -> Option<&'a HTMLContent> {
        if let Some(node) = self.state.pop_front() {
            if let HTMLContent::Tag(tag_box) = node { //add children to stack if tag object
                if self.is_pre_order {
                    for child in (**tag_box).content.iter().rev() {
                        if !((self.prune_closure)(child)) {
                            self.state.push_front(child);
                        }
                    }
                }
                else {
                    for child in (**tag_box).content.iter() {
                        if !((self.prune_closure)(child)) {
                            self.state.push_back(child);
                        }
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

impl<'a, F: Fn(&HTMLContent) -> bool> SearchTreeIter<'a, F> {
    pub fn new(is_pre_order: bool, prune_closure: F, content: &'a HTMLContent) -> Self {
        let mut temp_queue = VecDeque::new();
        if !((prune_closure)(content)) {
            temp_queue.push_back(content);
        }
        SearchTreeIter {
            is_pre_order,
            prune_closure,
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