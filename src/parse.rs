extern crate reqwest;
extern crate pest;
extern crate pest_derive;
use super::{
    tree::{ParseTree, Tag, HTMLContent},
};

use std::collections::HashMap;
use pest::iterators::Pair;
use pest::error::Error;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "html.pest"]
pub struct HTMLParser;

// ASSUMING THE FOLLOWING STRUCTS: 
// #[derive(Debug, PartialEq)]
// pub struct ParseTree<'a> {
//     pub root: HTMLContent<'a>,
// }

// #[derive(Debug, PartialEq)]
// pub struct Tag<'a> {
//     pub tag_type: &'a str,
//     pub attributes: HashMap<&'a str, &'a str>,
//     pub content: Vec<HTMLContent<'a>>,
// }

// #[derive(Debug, PartialEq)]
// pub enum HTMLContent<'a> {
//     Raw(&'a str),
//     Tag(Box<Tag<'a>>),
// }

pub fn get_and_parse_html(url: &str) -> Result<ParseTree, Error<Rule>> {
    parse_html(get_html(url))
}

fn get_html(url: &str) -> &str {
    // do error handling instead of unwrap later
    let _foo = reqwest::get(url).unwrap().text().unwrap();
    r#"<body>
        <div>
            <h1>foo</h1>
            <a>link</a>
        </div>
        <div>
            <p>baz<a>bar</a>qux</p>
        </div>
    </body>"#
}


/// Returns a `ParseTree` that parses the valid HTML.
/// 
/// # Example
/// use bsr::parse;
/// let raw_html = "<html></html>";
/// let parsed = parse_html(raw_html);
pub fn parse_html(unparsed_html: &str) -> Result<ParseTree, Error<Rule>> {
    let parsed_html = HTMLParser::parse(Rule::html, unparsed_html)?.next().unwrap();

    Ok(ParseTree {
        root: HTMLContent::Tag(Box::new(parse_html_element(parsed_html))),
    })
}

fn parse_html_element(html_element_rule: Pair<Rule>) -> Tag {
    match html_element_rule.as_rule() {
        Rule::html_element => {
            let mut inner_rule = html_element_rule.into_inner();
            let first_pair = inner_rule.next().unwrap();
            let first_rule = first_pair.as_rule();

            let (start_tag_type, attributes) = parse_start_tag(first_pair);

            match first_rule {
                Rule::self_closing_element => {
                    return Tag {
                        tag_type: start_tag_type,
                        attributes: attributes,
                        content: Vec::new(),
                    }
                },
                Rule::start_tag => {
                    let second_pair = inner_rule.next().unwrap();

                    match second_pair.as_rule() {
                        Rule::end_tag => {
                            // what is the idiomatic way of doing this
                            if start_tag_type == parse_end_tag(second_pair) {
                                return Tag {
                                    tag_type: start_tag_type,
                                    attributes: attributes,
                                    content: Vec::new(),
                                };
                            } else {
                                panic!("unmatched tags");
                            }
                        },
                        Rule::content => {
                            if start_tag_type == parse_end_tag(inner_rule.next().unwrap()) {
                                let mut contents = Vec::new();
                                let content_inner = second_pair.into_inner();

                                for content_pair in content_inner {
                                    match content_pair.as_rule() {
                                        Rule::raw => {
                                            let raw = content_pair.as_str().trim();
                                            if !raw.is_empty() {
                                                contents.push(HTMLContent::Raw(raw));
                                            }
                                        },
                                        Rule::html_element => {
                                            contents.push(HTMLContent::Tag(Box::new(
                                                parse_html_element(content_pair)
                                            )));
                                        },
                                        _ => unreachable!(),
                                    }
                                }
                                
                                return Tag {
                                    tag_type: start_tag_type,
                                    attributes: attributes,
                                    content: contents,
                                };
                            } else {
                                panic!("unmatched tags");
                            }
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}

fn parse_start_tag(start_rule: Pair<Rule>) -> (&str, HashMap<&str, &str>) {
    match start_rule.as_rule() {
        Rule::start_tag
        | Rule::self_closing_element => {
            let mut inner_rule = start_rule.into_inner();
            let start_tag_type = inner_rule.next().unwrap().as_str();
            let mut hm = HashMap::new();

            for attr in inner_rule {
                let mut attr = attr.into_inner();
                let name = attr.next().unwrap().as_str();
                let value = attr.next().unwrap().as_str();

                hm.insert(name, value);
            }

            (start_tag_type, hm)
        }
        _ => unreachable!(),
    }
}

fn parse_end_tag(end_rule: Pair<Rule>) -> &str {
    match end_rule.as_rule() {
        Rule::end_tag => end_rule.into_inner().as_str(),
        _ => unreachable!(),
    }
}


#[cfg(test)]
mod parse_tests {
    use super::*;

    fn generate_hashmap<'a>(tuples: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, &'a str> {
        let mut hm = HashMap::new();
        for tup in tuples {
            hm.insert(tup.0, tup.1);
        }

        hm
    }

    fn assert_element(element: Tag, tag_name: &str, attributes: Vec<(&str, &str)>, content: Vec<HTMLContent>) {
        let attrs = generate_hashmap(attributes);
        let e = Tag {
            tag_type: tag_name,
            attributes: attrs,
            content: content,
        };

        assert_eq!(element, e);
    }

    #[test]
    fn parse_end_tag_test() {
        assert_eq!("div", parse_end_tag(HTMLParser::parse(Rule::end_tag, "</div>")
                                        .unwrap().next().unwrap()));
    }

    #[test]
    fn parse_start_tag_attribute_test() {
        let hm = generate_hashmap(vec![("href", "google.com"), ("id", "foo")]);

        assert_eq!(("a", hm),
                    parse_start_tag(HTMLParser::parse(Rule::start_tag,
                                    "<a href=\"google.com\" id=\"foo\">")
                                    .unwrap().next().unwrap()));
    }

    #[test]
    fn parse_self_closing_tag_test() {
        let hm = generate_hashmap(vec![("href", "google.com"), ("foo", "bar")]);

        assert_eq!(("br", hm),
            parse_start_tag(HTMLParser::parse(Rule::self_closing_element, "<br href=\"google.com\" foo=\"bar\"/>")
                                .unwrap().next().unwrap()));
    }

    #[test]
    fn parse_element_empty_content_test() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div></div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        assert_element(res, "div", vec![], vec![]);
    }

    #[test]
    fn parse_element_raw_content_only_test() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div>FOO</div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        let c = HTMLContent::Raw("FOO");
        assert_element(res, "div", vec![], vec![c]);
    }

    #[test]
    fn parse_element_nested_test1() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div><p>FOO</p></div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        let foo = HTMLContent::Raw("FOO");
        let p = Tag {
            tag_type: "p",
            attributes: HashMap::new(),
            content: vec![foo],
        };
        let p = HTMLContent::Tag(Box::new(p));

        assert_element(res, "div", vec![], vec![p]);
    }

    #[test]
    fn parse_element_nested_test2() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div>HI<p>FOO</p>HELLO</div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        let foo = HTMLContent::Raw("FOO");
        let hi = HTMLContent::Raw("HI");
        let hello = HTMLContent::Raw("HELLO");

        let p = Tag {
            tag_type: "p",
            attributes: HashMap::new(),
            content: vec![foo],
        };
        let p = HTMLContent::Tag(Box::new(p));

        assert_element(res, "div", vec![], vec![hi, p, hello]);
    }

    #[test]
    fn parse_full_html_test1() {
        let raw_html =
            r#"<html>
            <head>
                <title>   TITLE  </title>
            </head>
            <body>
            SOME INITIAL TEXT
                <div id="id1">
                    <p>This is paragraph 1.</p>
                    <p>This is <br /> paragraph 2.</p>
                    <h3>Header!!</h3>
                    <a href="https://www.theatlantic.com/" id="link">Atlantic</a>
                </div>
                <div id="empty">

                </div>
            SOME FINAL TEXT
            </body>
            </html>"#;

        let res = parse_html(raw_html).unwrap();

        let title = HTMLContent::Raw("TITLE");
        let initial_text = HTMLContent::Raw("SOME INITIAL TEXT");
        let final_text = HTMLContent::Raw("SOME FINAL TEXT");
        let p1 = HTMLContent::Raw("This is paragraph 1.");
        let p2_1 = HTMLContent::Raw("This is");
        let p2_2 = HTMLContent::Raw("paragraph 2.");
        let h3 = HTMLContent::Raw("Header!!");
        let a = HTMLContent::Raw("Atlantic");

        let title = HTMLContent::Tag(Box::new(Tag {
            tag_type: "title",
            attributes: HashMap::new(),
            content: vec![title],
        }));
        let p1 = HTMLContent::Tag(Box::new(Tag {
            tag_type: "p",
            attributes: HashMap::new(),
            content: vec![p1],
        }));
        let br = HTMLContent::Tag(Box::new(Tag {
            tag_type: "br",
            attributes: HashMap::new(),
            content: vec![],
        }));
        let p2 = HTMLContent::Tag(Box::new(Tag {
            tag_type: "p",
            attributes: HashMap::new(),
            content: vec![p2_1, br, p2_2],
        }));
        let h3 = HTMLContent::Tag(Box::new(Tag {
            tag_type: "h3",
            attributes: HashMap::new(),
            content: vec![h3],
        }));
        let a = HTMLContent::Tag(Box::new(Tag {
            tag_type: "a",
            attributes: generate_hashmap(vec![("href", "https://www.theatlantic.com/"),
                                              ("id", "link")]),
            content: vec![a],
        }));

        let div1 = HTMLContent::Tag(Box::new(Tag {
            tag_type: "div",
            attributes: generate_hashmap(vec![("id", "id1")]),
            content: vec![p1, p2, h3, a],
        }));
        let div2 = HTMLContent::Tag(Box::new(Tag {
            tag_type: "div",
            attributes: generate_hashmap(vec![("id", "empty")]),
            content: vec![],
        }));

        let head = HTMLContent::Tag(Box::new(Tag {
            tag_type: "head",
            attributes: HashMap::new(),
            content: vec![title],
        }));
        let body = HTMLContent::Tag(Box::new(Tag {
            tag_type: "body",
            attributes: HashMap::new(),
            content: vec![initial_text, div1, div2, final_text],
        }));

        let parsed_html = ParseTree {
            root: HTMLContent::Tag(Box::new(
                Tag {
                tag_type: "html",
                attributes: HashMap::new(),
                content: vec![head, body],
            }))
        };


        assert_eq!(res, parsed_html);
    }

    #[test]
    fn parse_full_html_test2() {
        let raw_html = r#"<body>
            <div>
                <h1>foo</h1>
                <a>link</a>
            </div>
            <div>
                <p>baz<a>bar</a>qux</p>
            </div>
        </body>"#;

        let res = parse_html(raw_html).unwrap();

        let h1 = Tag {
            tag_type: "h1",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("foo")],
        };

        let a1 = Tag {
            tag_type: "a",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("link")],
        };

        let div1 = Tag {
            tag_type: "div",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(h1)), HTMLContent::Tag(Box::new(a1))],
        };

        let a2 = Tag {
            tag_type: "a",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("bar")],
        };

        let p = Tag {
            tag_type: "p",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Raw("baz"), HTMLContent::Tag(Box::new(a2)), HTMLContent::Raw("qux")],
        };

        let div2 = Tag {
            tag_type: "div",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(p))]
        };

        let body = Tag {
            tag_type: "body",
            attributes: HashMap::new(),
            content: vec![HTMLContent::Tag(Box::new(div1)),HTMLContent::Tag(Box::new(div2))]
        };

        let parse_tree = ParseTree {
            root: HTMLContent::Tag(Box::new(body))
        };

        assert_eq!(res, parse_tree);
    }
}