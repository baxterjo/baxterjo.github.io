#![allow(non_snake_case)]

use dioxus::prelude::*;
use pulldown_cmark::{Options, Parser};

#[derive(Props, Clone, PartialEq)]
pub struct MarkdownProps {
    #[props(default)]
    id: Signal<String>,
    #[props(default)]
    class: ReadOnlySignal<String>,

    content: ReadOnlySignal<String>,
}

/// Render some text as markdown.
pub fn Markdown(props: MarkdownProps) -> Element {
    let content = &*props.content.read();
    let parser = Parser::new_ext(content, Options::ENABLE_TABLES);

    let mut html_buf = String::new();
    pulldown_cmark::html::push_html(&mut html_buf, parser);

    rsx! {
        div {
            id: "{&*props.id.read()}",
            class: "{&*props.class.read()}",
            dangerous_inner_html: "{html_buf}",
        }
    }
}
