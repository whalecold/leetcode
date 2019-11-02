use clap;
use conf;
use problem;
use regex::Regex;
use std::fs;
use std::io::Write;
use std::path;

pub fn run(arg: &clap::ArgMatches) {
    if let Some(id) = arg.value_of("id") {
        // parse id
        let id = id.parse::<u32>().expect("get integer failed.");

        // get problem info
        let problem =
            problem::get_problem(id).expect(&format!("Error: failed to get problem {}", id));
        let code = problem
            .code_definition
            .iter()
            .filter(|&d| d.value == "rust")
            .next();
        if code.is_none() {
            println!("Problem {} has no rust version.", &id);
            return;
        }

        // format filename
        let code = code.unwrap();
        let file_name = format!("n{:04}_{}", id, problem.title_slug.replace("-", "_"));
        let new_mod_name = file_name.clone();
        let file_path = path::Path::new(conf::EXERCISE_PATH).join(format!(
            "{}{}",
            file_name,
            conf::FILE_SUFFIX
        ));
        if file_path.exists() {
            println!("problem {} already initialized", id);
            return;
        }

        // add mod to list
        let mod_line = fs::read_to_string(conf::MOD_FILE).unwrap();
        let mut mod_line: Vec<&str> = mod_line.split("\n").filter(|x| x.len() != 0).collect();
        let new_mod_name = &format!("mod {};", new_mod_name);
        mod_line.push(new_mod_name);
        mod_line.sort();
        fs::write(conf::MOD_FILE, mod_line.join("\n")).unwrap();

        // replace default value
        let template = fs::read_to_string(conf::TEMPLATE_FILE).unwrap();
        let code_content = template
            .replace(conf::TEMPLATE_PROBLEM_TITLE, &problem.title)
            .replace(conf::TEMPLATE_PROBLEM_DESC, &build_desc(&problem.content))
            .replace(conf::TEMPLATE_PROBLEM_DEFAULT_CODE, &code.default_code)
            .replace(conf::TEMPLATE_PROBLEM_ID, &format!("{}", id))
            .replace(
                conf::TEMPLATE_EXTRA_USE,
                &parse_extra_use(&code.default_code),
            );

        // write to file
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .unwrap();
        file.write_all(code_content.as_bytes()).unwrap();
        drop(file);
        println!("format file {:?} successfully", new_mod_name);
    } else {
        println!("Please supply a problem ID !!!");
    }
}

fn build_desc(content: &str) -> String {
    // TODO: fix this shit
    let content = content
        .replace("<strong>", "")
        .replace("</strong>", "")
        .replace("<em>", "")
        .replace("</em>", "")
        .replace("</p>", "")
        .replace("<p>", "")
        .replace("<b>", "")
        .replace("</b>", "")
        .replace("<pre>", "")
        .replace("</pre>", "")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "")
        .replace("</li>", "")
        .replace("<code>", "")
        .replace("</font>", "")
        .replace("</code>", "")
        .replace("<i>", "")
        .replace("</i>", "")
        .replace("<sub>", "")
        .replace("</sub>", "")
        .replace("</sup>", "")
        .replace("<sup>", "^")
        .replace("&nbsp;", " ")
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
        .replace("&minus;", "-")
        .replace("&#39;", "'")
        .replace("\n\n", "\n")
        .replace("\n", "\n * ");

    Regex::new("<font color=\"[a-z]*\">")
        .unwrap()
        .replace_all(&content, "")
        .to_string()
}

fn parse_extra_use(code: &str) -> String {
    let mut extra_use_line = String::new();
    // a linked-list problem
    if code.contains("pub struct ListNode") {
        extra_use_line.push_str("\nuse super::util::linked_list::{ListNode, to_list};")
    }
    if code.contains("pub struct TreeNode") {
        extra_use_line.push_str("\nuse super::util::tree::{TreeNode, to_tree};")
    }
    if code.contains("pub struct Point") {
        extra_use_line.push_str("\nuse super::util::point::Point;")
    }
    extra_use_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let re = Regex::new("<font color=\"[a-z]*\">").unwrap();
        assert_eq!(
            re.replace_all("1 1 <font color=\"red\">1 <font color=\"red\">1 1", ""),
            "1 1 1 1 1"
        );
    }
}
