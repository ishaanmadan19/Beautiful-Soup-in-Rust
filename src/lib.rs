

pub mod bsr {
    use std::collections::HashMap;
    extern crate reqwest;

    struct ParseTree<'a> {
        roots: Vec<HTMLContent>,
        stack: Option<Vec<&'a HTMLContent>>,
    }

    struct Tag {
        tag_type: String,
//        attributes: Vec<String>, // not sure which to use, hashmap bigger but clunkier
        attributes: HashMap<String, String>,
        content: Vec<HTMLContent>,
    }

    enum HTMLContent {
        Raw(String),
        Tag(Box<Tag>),
    }


    // Would be more clever and better to allow these functions to be applied on top of each other
    // I don't believe that works currently...

    // Also, should implement BFS iterator variant...

    impl ParseTree {

        pub fn new(url: &str) -> Self {
            parse_html(get_html(url))
        }

        fn get_html(url: &str) -> String {
            // do error handling instead of unwrap later
            reqwest::get(url).unwrap().text().unwrap()
        }

        fn parse_html(html: String) -> Self {
            ParseTree {
                roots: Vec::new(),
                stack: None,
            }
        }

        // should this return HTMLContent's or Tag's
        pub fn find_tags(&self, html_tag_type: String) -> impl Iterator<Item = HTMLContent> {

            // which soln is better, returning htmlcontents or tags

            // returns HTMLContent's
            self.iter().filter(move |node| match node { // do i need to deref the tag/maybe patter match of of box?
                                                HTMLContent::Raw(_) => false,
                                                HTMLContent::Tag(box_tag) => box_tag.tag_type == html_tag_type
                                            })
                // returns tags
//            self.iter().filter_map(|node|   if let HTMLContent::Tag(Box(html_tag)) = node { // i box pattern match needed/good?
//                if html_tag.tag_type == html_tag_type {
//                    Some(tag)
//                }
//                else {
//                    None
//                }
//            } else {
//                None
//            })

        }

        // Does this and the above automatically return &Item???
        pub fn find_text(&self) -> impl Iterator<Item = String> {
            // Which of the two is correct/better?

//            self.iter().map(move |node| match node {
//                HTMLContent::Raw(s) => s,
//                HTMLContent::Tag(box_tag) => None
//            });

            self.iter().filter_map(|node|   if let HTMLContent::Raw(s) = node {
                                                Some(s)
                                            } else {
                                                None
                                            })
        }

        // Again, return HTMLContent's or Tag's ?
        pub fn find_attrs(&self, attr: String, value: String) -> impl Iterator<Item = HTMLContent> {
            self.iter().filter(move |node| match node { // do i need to deref the tag/maybe patter match of of box?
                HTMLContent::Raw(_) => false,
                HTMLContent::Tag(box_tag) => {
                    match box_tag.attributes.get(&attr) {
                        Some(val) => val==value,
                        None => false
                    }
                }
            })
        }



    }

    // Do we also need into iterator to produce collections?
    impl<T> Iterator for ParseTree {
        // this is a DFS iterator...
        type Item = T;
        fn next(&mut self) -> Option<T> {
            let stack = match self.stack.as_mut() {
                None => {
                    let mut temp_stack = Vec::new();
                    for i in self.roots.iter().rev() {
                        temp_stack.push(i);
                    }
                    self.stack = Some(temp_stack);
                    self.stack.unwrap().as_mut()
                },
                Some(s) => s
            };
            if let Some(node) = stack.pop() {
                if let HTMLContent::Tag(tag_box) = node { //add children to stack if tag object
//                    let children = tag_box.content.iter().rev();
                    for child in (**tag_box).content.iter().rev() {
                        stack.push(child);
                    }
                }
                return Some(node)
            }
//            else {
            self.stack = None; // else is not necessary? is it idiomatic?
            None
//            }

        }

    }

}