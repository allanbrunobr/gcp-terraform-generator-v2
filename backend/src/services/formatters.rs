pub struct TerraformFormatter;

impl TerraformFormatter {
    pub fn format_resource_block(
        resource_type: &str,
        name: &str,
        attributes: Vec<(&str, &str)>
    ) -> String {
        let mut code = format!(r#"resource "{}" "{}" {{"#, resource_type, name);

        for (key, value) in attributes {
            code.push_str(&format!("\n  {} = {}", key, value));
        }

        code.push_str("\n}\n");
        code
    }
}