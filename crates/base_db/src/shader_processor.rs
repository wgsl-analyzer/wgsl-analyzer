use std::{collections::HashSet, ops::Range};

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
    pub fn process(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        mut emit_unconfigured: impl FnMut(Range<usize>, &str),
    ) -> String {
        self.process_inner(shader_str, shader_defs, &mut emit_unconfigured)
    }

    fn process_inner(
        &self,
        shader_str: &str,
        shader_defs: &HashSet<String>,
        emit_unconfigured: &mut dyn FnMut(Range<usize>, &str),
    ) -> String {
        let mut scopes = vec![(true, 0, "root scope")];
        let mut final_string = String::with_capacity(shader_str.len());

        for (line, offset) in lines_with_offsets(shader_str) {
            let use_line = if let Some(cap) = self.ifdef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && shader_defs.contains(def),
                    offset,
                    def,
                ));
                false
            } else if let Some(cap) = self.ifndef_regex.captures(line) {
                let def = cap.get(1).unwrap().as_str();
                scopes.push((
                    scopes.last().unwrap().0 && !shader_defs.contains(def),
                    offset,
                    def,
                ));
                false
            } else if self.else_regex.is_match(line) {
                let mut is_parent_scope_truthy = true;
                if scopes.len() > 1 {
                    is_parent_scope_truthy = scopes[scopes.len() - 2].0;
                }

                if let Some((last, start_offset, def)) = scopes.last_mut() {
                    if !*last {
                        let range = *start_offset..offset + line.len();
                        emit_unconfigured(range, def);
                    }

                    *start_offset = offset;
                    *last = is_parent_scope_truthy && !*last;
                }
                false
            } else if self.endif_regex.is_match(line) {
                if let Some((used, start_offset, def)) = scopes.pop() {
                    if !used {
                        let range = start_offset..offset + line.len();
                        emit_unconfigured(range, def);
                    }
                }

                if scopes.is_empty() {
                    // return Err(ProcessShaderError::TooManyEndIfs);
                }
                false
            } else {
                scopes.last().map(|&(used, _, _)| used).unwrap_or(true)
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

fn lines_with_offsets(input: &str) -> impl Iterator<Item = (&str, usize)> {
    input.lines().scan(0, |offset, line| {
        let the_offset = *offset;
        *offset = the_offset + line.len() + 1;

        Some((line, the_offset))
    })
}

#[cfg(test)]
mod tests {
    use super::ShaderProcessor;
    use std::collections::HashSet;

    fn test_shader(input: &str, defs: &[&str], output: &str) {
        let processor = ShaderProcessor::default();
        let defs = HashSet::from_iter(defs.iter().map(|s| s.to_string()));
        let result = processor.process(input, &defs, |_, _| {});

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
