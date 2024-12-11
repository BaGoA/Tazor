#![allow(dead_code)]

/// Kind of expression that we can parse
pub enum Expression {
    Raw(String),              // expression that we want only evaluate
    Variable(String, String), // expression defining a variable that we want store (name = definition)
}

impl Expression {
    pub fn new(expression: &str) -> Self {
        return match expression.split_once('=') {
            Some((name, definition)) => Self::Variable(
                String::from(name.trim_end()),
                String::from(definition.trim_start()),
            ),
            None => Self::Raw(String::from(expression)),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_new_with_raw_expression() {
        let expression: String = String::from("1 + 1");

        match Expression::new(expression.as_str()) {
            Expression::Raw(raw_expression) => assert_eq!(raw_expression, expression),
            Expression::Variable(_, _) => assert!(false),
        }
    }

    #[test]
    fn test_expression_new_with_variable_definition() {
        let variable_name: String = String::from("x");
        let variable_definition: String = String::from("1 + 1");

        let expression: String = format!("{} = {}", variable_name, variable_definition);

        match Expression::new(expression.as_str()) {
            Expression::Raw(_) => assert!(false),
            Expression::Variable(name, definition) => {
                assert_eq!(name, variable_name);
                assert_eq!(definition, variable_definition);
            }
        }
    }
}
