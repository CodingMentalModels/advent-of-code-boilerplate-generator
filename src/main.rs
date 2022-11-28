use std::path::Path;

use handlebars::Handlebars;
use serde_json::json;
use template::Templatizer;

mod template;

const OUTPUT_PATH: &str = "C:/Users/cmsdu/repos/advent-of-code-2022/";
const TEMPLATE_PATH: &str = "C:/Users/cmsdu/repos/advent-of-code-boilerplate-generator/templates/";

fn main() {
    let templatizer = Templatizer::try_new(Path::new(TEMPLATE_PATH)).unwrap();

    let input_path = Path::join(Path::new(OUTPUT_PATH), "input/input.txt");
    let input_output_path = Path::join(Path::new(OUTPUT_PATH), "src/input/input.rs");
    templatizer.render_template_to_file("input.txt", &json!({"filepath": input_path}), &input_output_path).unwrap();

    let input_mod_path = Path::join(Path::new(OUTPUT_PATH), "src/input/mod.rs");
    templatizer.render_template_to_file("mod.txt", &json!({"modules": ["input"]}), &input_mod_path).unwrap();

    let problem_mod_path = Path::join(Path::new(OUTPUT_PATH), "src/problems/mod.rs");
    templatizer.render_template_to_file("mod.txt", &json!({"modules": (1..26).into_iter().map(|x| format!("{}{:02}", "problem_", x)).collect::<Vec<_>>()}), &problem_mod_path).unwrap();

    for i in 1..26 {
        let problem_path = Path::join(Path::new(OUTPUT_PATH), &format!("src/problems/problem_{:02}.rs", i));
        templatizer.render_template_to_file("problem.txt", &json!({"problem_number": format!("{:02}", i)}), &problem_path).unwrap();
    }
}

#[cfg(test)]
mod test_advent_of_code_boilerplate_generator {
    use super::*;

    #[test]
    fn test_basic_handlebars_example() {
        
        let mut reg = Handlebars::new();
        let result = reg.render_template("Hello {{name}}", &json!({"name": "foo"}));
        assert_eq!(result.unwrap(), "Hello foo");
        
        reg.register_template_string("tpl_1", "Good afternoon, {{name}}").unwrap();

        assert_eq!(reg.render("tpl_1", &json!({"name": "foo"})).unwrap(), "Good afternoon, foo".to_string());
        
    }
}
