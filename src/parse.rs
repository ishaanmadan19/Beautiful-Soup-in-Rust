use std::collections::HashMap;
use pest::iterators::Pair;
use pest::error::Error;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "html.pest"]
struct HTMLParser;

#[derive(Debug, PartialEq)]
struct ParseTree {
    root: HTMLContent,
}

#[derive(Debug, PartialEq)]
struct Tag {
    tag_type: String,
    attributes: HashMap<String, String>,
    content: Vec<HTMLContent>,
}

#[derive(Debug, PartialEq)]
enum HTMLContent {
    Raw(String),
    Tag(Box<Tag>),
}

// fn main() {
//     let html = r#"<html></html>"#;
//     let res = parse_html(html).unwrap();
//     println!("{:?}", res);
// }

/// MAIN FUNCTION:
/// See parse_full_html_test1 and parse_full_html_test2 for examples.
fn parse_html(unparsed_html: &str) -> Result<ParseTree, Error<Rule>> {
    let parsed_html = HTMLParser::parse(Rule::html, unparsed_html)?.next().unwrap();

    Ok(ParseTree {
        root: HTMLContent::Tag(Box::new(parse_html_element(parsed_html))),
    })
}

fn parse_html_element(html_element_rule: Pair<Rule>) -> Tag {
    match html_element_rule.as_rule() {
        Rule::html_element => {
            let mut inner_rule = html_element_rule.into_inner();
            let start_tag = inner_rule.next().unwrap();

            // need to parse start_tag in either case
            let (start_tag_type, attributes) = parse_start_tag(start_tag);

            let next_pair = inner_rule.next().unwrap();

            match next_pair.as_rule() {
                // case 1: html element empty
                Rule::end_tag => {
                    // what is the idiomatic way of doing this
                    if start_tag_type == parse_end_tag(next_pair) {
                        return Tag {
                            tag_type: start_tag_type,
                            attributes: attributes,
                            content: vec![HTMLContent::Raw(String::from(""))],
                        };
                    } else {
                        panic!("unmatched tags");
                    }
                },
                // case 2: content
                Rule::content => {
                    if start_tag_type == parse_end_tag(inner_rule.next().unwrap()) {
                        let mut contents = Vec::new();

                        let content_inner = next_pair.into_inner();
                        for content_pair in content_inner {
                            match content_pair.as_rule() {
                                // content is raw text
                                Rule::raw => {
                                    let raw = content_pair.as_str().trim();
                                    if !raw.is_empty() {
                                        contents.push(HTMLContent::Raw(String::from(raw)));
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
}

fn parse_start_tag(start_rule: Pair<Rule>) -> (String, HashMap<String, String>) {
    match start_rule.as_rule() {
        Rule::start_tag => {
            let mut inner_rule = start_rule.into_inner();
            let start_tag_type = inner_rule.next().unwrap().as_str().to_owned();
            let mut hm = HashMap::new();

            for attr in inner_rule {
                let mut attr = attr.into_inner();
                let name = attr.next().unwrap().as_str().to_owned();
                let value = attr.next().unwrap().as_str().to_owned();

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
    
    fn generate_hashmap(tuples: Vec<(&str, &str)>) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        for tup in tuples {
            hm.insert(String::from(tup.0), String::from(tup.1));
        }

        hm
    }

    fn assert_element(element: Tag, tag_name: &str, attributes: Vec<(&str, &str)>, content: Vec<HTMLContent>) {
        let attrs = generate_hashmap(attributes);

        let e = Tag {
            tag_type: String::from(tag_name),
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
        let mut hm = HashMap::new();
        hm.insert(String::from("href"), String::from("google.com"));
        hm.insert(String::from("id"), String::from("foo"));

        assert_eq!(("a".to_owned(), hm),
                    parse_start_tag(HTMLParser::parse(Rule::start_tag,
                                    "<a href=\"google.com\" id=\"foo\">")
                                    .unwrap().next().unwrap()));
    }

    #[test]
    fn parse_element_empty_content_test() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div></div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        let c = HTMLContent::Raw(String::from(""));
        assert_element(res, "div", vec![], vec![c]);
    }

    #[test]
    fn parse_element_raw_content_only_test() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div>FOO</div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);
        
        let c = HTMLContent::Raw(String::from("FOO"));
        assert_element(res, "div", vec![], vec![c]);
    }

    #[test]
    fn parse_element_nested_test1() {
        let html_element = HTMLParser::parse(Rule::html_element,
                            "<div><p>FOO</p></div>")
                            .unwrap().next().unwrap();
        let res = parse_html_element(html_element);

        let foo = HTMLContent::Raw(String::from("FOO"));
        let p = Tag {
            tag_type: String::from("p"),
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

        let foo = HTMLContent::Raw(String::from("FOO"));
        let hi = HTMLContent::Raw(String::from("HI"));
        let hello = HTMLContent::Raw(String::from("HELLO"));

        let p = Tag {
            tag_type: String::from("p"),
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
                    <p>This is paragraph 2.</p>
                    <h3>Header!!</h3>
                    <a href="https://www.theatlantic.com/" id="link">Atlantic</a>
                </div>
                <div id="empty">
                    
                </div>
            SOME FINAL TEXT
            </body>
            </html>"#;

        let res = parse_html(raw_html).unwrap();

        let title = HTMLContent::Raw(String::from("TITLE"));
        let initial_text = HTMLContent::Raw(String::from("SOME INITIAL TEXT"));
        let final_text = HTMLContent::Raw(String::from("SOME FINAL TEXT"));
        let p1 = HTMLContent::Raw(String::from("This is paragraph 1."));
        let p2 = HTMLContent::Raw(String::from("This is paragraph 2."));
        let h3 = HTMLContent::Raw(String::from("Header!!"));
        let a = HTMLContent::Raw(String::from("Atlantic"));
        let empty = HTMLContent::Raw(String::from(""));

        let title = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("title"),
            attributes: HashMap::new(),
            content: vec![title],
        }));
        let p1 = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("p"),
            attributes: HashMap::new(),
            content: vec![p1],
        }));
        let p2 = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("p"),
            attributes: HashMap::new(),
            content: vec![p2],
        }));
        let h3 = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("h3"),
            attributes: HashMap::new(),
            content: vec![h3],
        }));
        let a = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("a"),
            attributes: generate_hashmap(vec![("href", "https://www.theatlantic.com/"),
                                              ("id", "link")]),
            content: vec![a],
        }));

        let div1 = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("div"),
            attributes: generate_hashmap(vec![("id", "id1")]),
            content: vec![p1, p2, h3, a],
        }));
        let div2 = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("div"),
            attributes: generate_hashmap(vec![("id", "empty")]),
            content: vec![empty],
        }));

        let head = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("head"),
            attributes: HashMap::new(),
            content: vec![title],
        }));
        let body = HTMLContent::Tag(Box::new(Tag {
            tag_type: String::from("body"),
            attributes: HashMap::new(),
            content: vec![initial_text, div1, div2, final_text],
        }));

        let parsed_html = ParseTree {
            root: HTMLContent::Tag(Box::new(
                Tag {
                tag_type: String::from("html"),
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

        let parse_tree = ParseTree {
            root: HTMLContent::Tag(Box::new(body))
        };

        assert_eq!(res, parse_tree);
    }
}