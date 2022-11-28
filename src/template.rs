use std::{path::Path, fs};

use handlebars::Handlebars;
use serde_json::json;


pub struct Templatizer<'a> {
    handlebars: Handlebars<'a>,
}

impl<'a> Templatizer<'a> {

    pub fn try_new(path: &Path) -> Result<Self, String> {
        let handlebars = Self::load_templates(path)?;
        
        Ok(
            Self {
                handlebars,
            }
        )
    }
    
    fn load_templates(path: &Path) -> Result<Handlebars<'a>, String> {
        let mut handlebars = Handlebars::new();
        
        let result: Result<(), String> = fs::read_dir(path).map_err(|_| "Unable to read directory.".to_string())?
            .map(|entry_result| {
                    entry_result.map(
                        |entry| {
                            let path = entry.path();
                            let file_name = path.file_name().unwrap().to_str().unwrap();
                            handlebars.register_template_file(file_name, entry.path()).unwrap();
                        }
                    ).map_err(|_| "Unable to open file.".to_string())
            }).collect();

        result.map(|_| handlebars)
        
    }

    pub fn len(&self) -> usize {
        self.handlebars.get_templates().len()
    }

    pub fn render_template(&self, template_name: &str, data: &serde_json::Value) -> Result<String, String> {
        self.handlebars.render(template_name, data).map_err(|_| "Unable to render template.".to_string())
    }

    pub fn render_template_to_file(&self, template_name: &str, data: &serde_json::Value, output_path: &Path) -> Result<(), String> {
        let rendered = self.render_template(template_name, data)?;
        fs::write(output_path, rendered).map_err(|_| "Unable to write to file.".to_string())
    }

}

#[cfg(test)]
mod test_template_loading_and_rendering {
    use std::path::Path;

    use crate::TEMPLATE_PATH;

    use super::*;

    #[test]
    fn test_input_template_loads_and_renders() {
        
        let templatizer = Templatizer::try_new(Path::new(TEMPLATE_PATH)).unwrap();
        assert_eq!(templatizer.len(), 3);

        let templatized_input = templatizer.render_template(
            "input.txt", &json!({"filepath": "foo"})
        ).unwrap();
        
        assert_eq!(
            templatized_input.len(),
            2602
        );

        let line_0 = templatized_input.lines().nth(0).unwrap();
        let line_1 = templatized_input.lines().nth(1).unwrap();
        let line_2 = templatized_input.lines().nth(2).unwrap();
        
        assert_eq!(line_0, "use std::{fs::File, io::Read, path::Path};");
        assert_eq!(line_1, "");
        assert_eq!(line_2, "const INPUT_PATH_HEAD: &str = \"foo\";");        
    }

    #[test]
    fn test_mod_template_loads_and_renders() {

        let templatizer = Templatizer::try_new(Path::new(TEMPLATE_PATH)).unwrap();
        assert_eq!(templatizer.len(), 3);

        let templatized_mod = templatizer.render_template(
            "mod.txt",
            &json!({"modules": (1..3).into_iter().map(|x| format!("{}{:02}", "problem_", x)).collect::<Vec<_>>()})
        ).unwrap();

        assert_eq!(templatized_mod.lines().count(), 4);
        assert_eq!(templatized_mod.lines().next(), Some("pub mod problem_01;"));
        assert_eq!(templatized_mod.lines().last(), Some(""));
    }

    #[test]
    fn test_problem_template_loads_and_renders() {

        let templatizer = Templatizer::try_new(Path::new(TEMPLATE_PATH)).unwrap();
        assert_eq!(templatizer.len(), 3);

        let templatized_problem = templatizer.render_template(
            "problem.txt",
            &json!({"problem_number": "01"})
        ).unwrap();
       
        assert_eq!(templatized_problem.lines().count(), 39);
        assert_eq!(templatized_problem.lines().nth(28), Some("    fn test_problem_01b_passes() {"));
    }
}