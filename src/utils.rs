// use log::debug;
// use markdown::{CompileOptions, ParseOptions};
// use std::fs::read_to_string;
// pub fn load_markdown_file(path: &str) -> String {
//     let file_str = read_to_string(path).unwrap();
//     let options = markdown::Options {
//         parse: ParseOptions::default(),
//         compile: CompileOptions {
//             allow_dangerous_html: true,
//             ..Default::default()
//         },
//     };
//     let html = markdown::to_html_with_options(file_str.as_str(), &options).unwrap();
//     debug!("HTML: {html}");
//     let m_tree = markdown::to_mdast(file_str.as_str(), &markdown::ParseOptions::default());
//     debug!("MTREE: {m_tree:#?}");
//     html
// }

// #[cfg(test)]
// mod test {
//     use super::load_markdown_file;

//     #[test]
//     fn test_fn_works() {
//         let _ = env_logger::try_init();
//         let path = concat!(
//             env!("CARGO_MANIFEST_DIR"),
//             "/public/content/projects/alarm_clock/content.md"
//         );
//         load_markdown_file(path);
//     }
// }
