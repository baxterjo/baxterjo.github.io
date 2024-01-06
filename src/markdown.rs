#![allow(non_snake_case)]

use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser};

#[derive(Props)]
pub struct MarkdownProps<'a> {
    #[props(default)]
    id: &'a str,
    #[props(default)]
    class: &'a str,

    content: &'a str,
}

/// Render some text as markdown.
pub fn Markdown<'a>(cx: Scope<'a, MarkdownProps<'a>>) -> Element {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(cx.props.content, options);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    cx.render(rsx! {
        div {
            id: "{cx.props.id}",
            class: "{cx.props.class}",
            dangerous_inner_html: "{html_buf}"
        }
    })
}
