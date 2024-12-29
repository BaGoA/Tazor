#![allow(dead_code)]

use std::collections::HashMap;

/// Kind of expression that we can parse
pub enum Expression {
    Raw(String),              // expression that we want only evaluate
    Variable(String, String), // expression defining a variable that we want store (name = definition)
}

impl Expression {
    /// Construct an Expression from string
    /// We can have following case:
    ///   - raw expression as '1 + 1' or 'cos(pi) * sqrt(2)'
    ///   - expression defining a variable like this 'x = 1 + 1'
    ///  where left side of equality is its name and right side is its definition
    pub fn new(expression: &str) -> Self {
        return match expression.split_once('=') {
            Some((name, definition)) => Self::Variable(
                String::from(name.trim_end()),
                String::from(definition.trim_start()),
            ),
            None => Self::Raw(String::from(expression)),
        };
    }

    /// Replace all variable contained in expression by their value
    /// The variables are given in argument through HashMap where
    /// pair (key, value) correspond respectively to name and value of variable
    pub fn replace_variables(&mut self, variables: &HashMap<String, f64>) {
        let definition: &mut String = match self {
            Self::Raw(raw_expression) => raw_expression,
            Self::Variable(_, definition) => definition,
        };

        for (variable_name, variable_value) in variables {
            let mut replaced_definition: String =
                definition.replace(variable_name, format!("{}", variable_value).as_str());

            let _ = std::mem::swap(definition, &mut replaced_definition);
        }
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

    #[test]
    fn test_expression_replace_variables_in_raw_expression() {
        let mut variables: HashMap<String, f64> = HashMap::new();

        variables.insert(String::from("x"), 1.0);
        variables.insert(String::from("velocity"), 3.43);
        variables.insert(String::from("time"), 5.9954);

        let raw_expression: String = String::from("(x - 2.75) + velocity * time");

        let replaced_raw_expression: String = format!(
            "({} - 2.75) + {} * {}",
            variables["x"], variables["velocity"], variables["time"]
        );

        let mut expression: Expression = Expression::new(raw_expression.as_str());
        expression.replace_variables(&variables);

        match expression {
            Expression::Raw(replaced_expression) => {
                assert_eq!(replaced_raw_expression, replaced_expression)
            }
            Expression::Variable(_, _) => assert!(false),
        }
    }

    #[test]
    fn test_expression_replace_variables_in_variable_expression() {
        let mut variables: HashMap<String, f64> = HashMap::new();

        variables.insert(String::from("x"), 1.0);
        variables.insert(String::from("velocity"), 3.43);
        variables.insert(String::from("time"), 5.9954);

        let raw_expression: String = String::from("y = (x - 2.75) + velocity * time");

        let replaced_raw_expression: String = format!(
            "({} - 2.75) + {} * {}",
            variables["x"], variables["velocity"], variables["time"]
        );

        let mut expression: Expression = Expression::new(raw_expression.as_str());
        expression.replace_variables(&variables);

        match expression {
            Expression::Raw(_) => assert!(false),
            Expression::Variable(_, replaced_expression) => {
                assert_eq!(replaced_raw_expression, replaced_expression)
            }
        }
    }
}
