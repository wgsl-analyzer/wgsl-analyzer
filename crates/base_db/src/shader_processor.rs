use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

pub static SHADER_PROCESSOR: Lazy<ShaderProcessor> = Lazy::new(ShaderProcessor::default);

pub struct ShaderProcessor {
    ifdef_regex: Regex,
    ifndef_regex: Regex,
    else_regex: Regex,
    endif_regex: Regex,
}

impl Default for ShaderProcessor {
    fn default() -> Self {
        Self {
            ifdef_regex: Regex::new(r"^\s*#\s*ifdef\s*([\w|\d|_]+)").unwrap(),
            ifndef_regex: Regex::new(r"^\s*#\s*ifndef\s*([\w|\d|_]+)").unwrap(),
            else_regex: Regex::new(r"^\s*#\s*else").unwrap(),
            endif_regex: Regex::new(r"^\s*#\s*endif").unwrap(),
        }
    }
}

impl ShaderProcessor {
    pub fn process(&self, shader_str: &str, shader_defs: &HashSet<String>) -> String {
        let mut scopes = vec![true];
        let mut final_string = String::with_capacity(shader_str.len());
        for line in shader_str.lines() {
            let use_line = if let Some(cap) = self.ifdef_regex.captures(line) {
                let def = cap.get(1).unwrap();
                scopes.push(*scopes.last().unwrap() && shader_defs.contains(def.as_str()));
                false
            } else if let Some(cap) = self.ifndef_regex.captures(line) {
                let def = cap.get(1).unwrap();
                scopes.push(*scopes.last().unwrap() && !shader_defs.contains(def.as_str()));
                false
            } else if self.else_regex.is_match(line) {
                let mut is_parent_scope_truthy = true;
                if scopes.len() > 1 {
                    is_parent_scope_truthy = scopes[scopes.len() - 2];
                }
                if let Some(last) = scopes.last_mut() {
                    *last = is_parent_scope_truthy && !*last;
                }
                false
            } else if self.endif_regex.is_match(line) {
                scopes.pop();
                if scopes.is_empty() {
                    // return Err(ProcessShaderError::TooManyEndIfs);
                }
                false
            } else {
                scopes.last().copied().unwrap_or(true)
            };

            if use_line {
                final_string.push_str(line);
            } else {
                final_string.extend(std::iter::repeat(' ').take(line.len()));
            }
            final_string.push('\n');
        }

        if scopes.len() != 1 {
            // return Err(ProcessShaderError::NotEnoughEndIfs);
        }

        final_string
    }
}

#[cfg(test)]
mod tests {
    use super::ShaderProcessor;
    use std::collections::HashSet;

    fn test_shader(input: &str, defs: &[&str], output: &str) {
        let processor = ShaderProcessor::default();
        let defs = HashSet::from_iter(defs.iter().map(|s| s.to_string()));
        let result = processor.process(input, &defs);

        assert_eq!(result, output);
    }

    #[test]
    fn test_empty() {
        test_shader(
            r#"
"#,
            &[],
            r#"
"#,
        );
    }

    #[test]
    fn test_false_replace_str() {
        test_shader(
            r#"
.
#ifdef FALSE
IGNORE
#endif
.
"#,
            &[],
            r#"
.
            
      
      
.
"#,
        );
    }
}
