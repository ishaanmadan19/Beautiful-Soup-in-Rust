
pub mod bsr {

    // if we do a streaming/event based solution. it says: parsing html is painn but don;t want to
    // always build a tree. maybe build a specific kinds of tree. so use the event based to build the tree


    // later on could add tag specific caches for quick access to all anchors, images, text, etc...
    // would be some HashMap of tag type and list of pointers to Elements
    pub struct BSRObject {
        // Or could separate by meta, head, body ...
        // <!DOCTYPE HTML>
        // <html lang="en">
        // <head, <meta's, <title, <body
        roots: Vec<Box<element>>,
    }

    pub struct Element {
        tag: String,
        attributes: Vec<(String,String)>,
        children: Vec<Box<element>>,
    }

    impl Element {
        // do attrs and children later...
        pub fn new(t: &str) -> Box<Self> {
            Box::new(Element {
                tag: t.to_owned(),
                attributes: Vec::new(),
                children: Vec::new(),
            })
        }

//        pub fn addChild(&mut self, el: Box<Self>) {
//            self.children.push(el);
//        }

        pub fn addChildren(&mut self, els: Vec<Box<Self>>) {
            self.children = els;
        }
    }

    impl BRSObbject {

        pub fn new(url: &str) -> Self {
            BSRObject {
                roots: parseHTML(url),
            }
        }

        // Start with just body parse and then implement whole doc
        fn parseHTML(url: &str) -> Vec<Box<Element>> {
            parseHelper(get_html(url),0)
        }

        // recursive could be risky for blowing stack... but html nesting seems tractable
        // ALSO NEED TO BE CAREFUL OF LOOSE CARROTS, IN QUOTES CASE IS EASY, NO QUOTES IS HARD...
        fn parseHelper(html: &str, start: usize) -> Vec<Box<Element>> {
            let mut root = Vec::new();

            // better to just keep pointers or use one iter?

            // assume properly formed and only non-self-closing tags for now...
            let mut last = start;
            while let Some(i) = html[start..].find("<") { // handle none cases...
                let Some(j) = html[i..].find(">");
                // if slice contains / no recur else recur? with helper
                if !html[i..j].contains("/") {
                    root.push(Element::new(&html[(i+1)..j])); // might have off by 1
                    root.last().unwrap().addChildren(parseHelper(html, j));
                }
                    //WE DO NEED RETURN int in recur to skip ahead to sibling elements

                else { // break if next carrot couple is also closure bc end of nest
                    let Some(next_tag) = html[j..].find("<");
                    if html.get(next_tag + 1) == "/" {
                        break;
                    }
                }
            }

            root
        }

        // find api to pull raw html from a given web page
        // https://docs.rs/reqwest/0.8.6/reqwest/struct.Response.html
        fn get_html(url: &str) -> &str {
            return "<body><h1></h1><a><h1></h1></a></body>"
                //"<body><div><a>foo</a><a><div>bar</div></a></div><div><h1>baz</h1></div><div><a>foobar</a></div></body>"
        }
    }

}