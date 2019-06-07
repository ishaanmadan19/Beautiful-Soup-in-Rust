

pub mod rusty_soup {
    use std::collections::HashMap;
    use std::collections::VecDeque;
//    extern crate reqwest;

    pub struct ParseTree {
        root: HTMLContent,
    }

    pub struct Tag {
        tag_type: String,
        attributes: HashMap<String, String>,
        content: Vec<HTMLContent>,
    }

    pub enum HTMLContent {
        Raw(String),
        Tag(Box<Tag>),
    }

    pub struct PtPreOrderIter<'a> {
        stack: Vec<&'a HTMLContent>,
    }

    pub struct PtLevelOrderIter<'a> {
        queue: VecDeque<&'a HTMLContent>,
    }

    impl Tag {
        pub fn get_tag(&self) -> &String {
            &self.tag_type
        }
    }

    impl ParseTree {

        pub fn new(url: &str) -> Self {
            ParseTree {
                root: ParseTree::parse_html(ParseTree::get_html(url)),
            }
        }

        fn get_html(url: &str) -> String {
            // do error handling instead of unwrap later
//            reqwest::get(url).unwrap().text().unwrap()
            "foobar".to_owned()
        }

        fn parse_html(html: String) -> HTMLContent {
            HTMLContent::Raw("link".to_owned())

        }

        pub fn pre_iter(&self) -> impl Iterator<Item = &HTMLContent> {
            PtPreOrderIter::new(&self.root)
        }

        pub fn level_iter(&self) -> impl Iterator<Item = &HTMLContent> {
            PtLevelOrderIter::new(&self.root)
        }

    }

    impl HTMLContent {
        pub fn pre_iter(&self) -> impl Iterator<Item = &HTMLContent> {
            PtPreOrderIter::new(self)
        }

        pub fn level_iter(&self) -> impl Iterator<Item = &HTMLContent> {
            PtLevelOrderIter::new(self)
        }
    }

    impl<'a> PtPreOrderIter<'a> {
        pub fn new(content: &'a HTMLContent) -> Self {
            PtPreOrderIter {
                stack: vec![content],
            }
        }

        // why can't use &self
        pub fn find_tags(self, html_tag_type: String) -> impl Iterator<Item = &'a HTMLContent> {
            self.filter(move |node| match node { // do i need to deref the tag/maybe patter match of of box?
                HTMLContent::Raw(_) => false,
                HTMLContent::Tag(box_tag) => box_tag.tag_type == html_tag_type
            })
        }

        // why can't use &self
        pub fn find_tags2(self, html_tag_type: String) -> impl Iterator<Item = &'a Tag> {
            self.filter_map(move|node|   if let HTMLContent::Tag(box_tag) = node { // i box pattern match needed/good?
                if (**box_tag).tag_type == html_tag_type {
                    Some(&(**box_tag))
                }
                else {
                    None
                }
            } else {
                None
            })
        }

//        // Does this and the above automatically return &Item???
//        pub fn find_text(&self) -> impl Iterator<Item = String> {
//            // Which of the two is correct/better?
//
////            self.iter().map(move |node| match node {
////                HTMLContent::Raw(s) => s,
////                HTMLContent::Tag(box_tag) => None
////            });
//
//            self.iter().filter_map(|node|   if let HTMLContent::Raw(s) = node {
//                Some(s)
//            } else {
//                None
//            })
//        }

//        // Again, return HTMLContent's or Tag's ?
//        pub fn find_attrs(&self, attr: String, value: String) -> impl Iterator<Item = HTMLContent> {
//            self.iter().filter(move |node| match node { // do i need to deref the tag/maybe patter match of of box?
//                HTMLContent::Raw(_) => false,
//                HTMLContent::Tag(box_tag) => {
//                    match box_tag.attributes.get(&attr) {
//                        Some(val) => *val==value,
//                        None => false
//                    }
//                }
//            })
//        }
    }

    impl<'a> Iterator for PtPreOrderIter<'a> {
        type Item = &'a HTMLContent;
        fn next(&mut self) -> Option<&'a HTMLContent> {
            if let Some(node) = self.stack.pop() {
                if let HTMLContent::Tag(tag_box) = node { //add children to stack if tag object
                    for child in (**tag_box).content.iter().rev() {
                        self.stack.push(child);
                    }
                }
                return Some(node)
            }
            else {
                None
            }

        }
    }

    impl<'a> PtLevelOrderIter<'a> {
        pub fn new(content: &'a HTMLContent) -> Self {
            let mut temp_queue = VecDeque::new();
            temp_queue.push_back(content);
            PtLevelOrderIter {
                queue: temp_queue,
            }
        }
    }

    impl<'a> Iterator for PtLevelOrderIter<'a> {
        type Item = &'a HTMLContent;
        fn next(&mut self) -> Option<&'a HTMLContent> {
            if let Some(node) = self.queue.pop_front() {
                if let HTMLContent::Tag(tag_box) = node {
                    for child in (**tag_box).content.iter() {
                        self.queue.push_back(child);
                    }
                }
                return Some(node)
            }
            else {
                None
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

}