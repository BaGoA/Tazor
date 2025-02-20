#![allow(dead_code)]

use std::collections::HashMap;

/// Kind of expression that we can parse
pub enum Expression {
    Raw(String),                           // expression that we want only evaluate
    Variable(String, String), // expression defining a variable that we want store (name = definition)
    Function(String, Vec<String>, String), // expression defining a function that we want store (name: x, y = definition)
}

impl Expression {
    /// Construct an Expression from string
    /// We can have following case:
    ///   - raw expression as '1 + 1' or 'cos(pi) * sqrt(2)'
    ///   - expression defining a variable like this 'x = 1 + 1'
    ///   - expression defining a function like this 'f: x, y = x * x + y * y
    ///  where left side of equality is its name and right side is its definition
    pub fn new(expression: &str) -> Self {
        return match expression.split_once('=') {
            // Here the expression define a variable or function
            Some((name, definition)) => match name.split_once(':') {
                // Here we have a function
                Some((fun_name, fun_variables_compact)) => {
                    let fun_variables: Vec<String> = fun_variables_compact
                        .split(',')
                        .map(|fun_variable_name: &str| {
                            String::from(fun_variable_name.trim_start().trim_end())
                        })
                        .collect();

                    return Self::Function(
                        String::from(fun_name.trim_start().trim_end()),
                        fun_variables,
                        String::from(definition.trim_start().trim_end()),
                    );
                }
                // Here we have a variable
                None => Self::Variable(
                    String::from(name.trim_start().trim_end()),
                    String::from(definition.trim_start().trim_end()),
                ),
            },
            // Here we have a raw expression
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
            Self::Function(_, _, definition) => definition,
        };

        for (variable_name, variable_value) in variables {
            let mut replaced_definition: String =
                definition.replace(variable_name, format!("{}", variable_value).as_str());

            let _ = std::mem::swap(definition, &mut replaced_definition);
        }
    }

    /// Replace all function contained in expression by their definition
    /// The function are given in argument through HashMap where
    /// key correspond to name of function and value is a pair containing
    /// name of variables and definition of function
    pub fn replace_functions(&mut self, _functions: &HashMap<String, (Vec<String>, String)>) {
        // TODO
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
            _ => assert!(false),
        }
    }

    #[test]
    fn test_expression_new_with_variable_definition() {
        let variable_name: String = String::from("x");
        let variable_definition: String = String::from("1 + 1");

        let expression: String = format!("{} = {}", variable_name, variable_definition);

        match Expression::new(expression.as_str()) {
            Expression::Variable(name, definition) => {
                assert_eq!(name, variable_name);
                assert_eq!(definition, variable_definition);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_expression_new_with_function_definition() {
        let function_name: String = String::from("distance");
        let function_variables: Vec<String> =
            vec![String::from("x"), String::from("y"), String::from("z)")];
        let function_definition: String = String::from("x * x + y * y + z * z");

        let expression: String = format!(
            "{}: {}, {}, {} = {}",
            function_name,
            function_variables[0],
            function_variables[1],
            function_variables[2],
            function_definition
        );

        match Expression::new(&expression.as_str()) {
            Expression::Function(name, variables, definition) => {
                assert_eq!(name, function_name);
                assert_eq!(variables, function_variables);
                assert_eq!(definition, function_definition);
            }
            _ => assert!(false),
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
            _ => assert!(false),
        }
    }

    #[test]
    fn test_expression_replace_variables_in_variable_expression() {
        let mut variables: HashMap<String, f64> = HashMap::new();

        variables.insert(String::from("x"), 1.0);
        variables.insert(String::from("velocity"), 3.43);
        variables.insert(String::from("time"), 5.9954);

        let var_expression: String = String::from("y = (x - 2.75) + velocity * time");

        let replaced_var_expression: String = format!(
            "({} - 2.75) + {} * {}",
            variables["x"], variables["velocity"], variables["time"]
        );

        let mut expression: Expression = Expression::new(var_expression.as_str());
        expression.replace_variables(&variables);

        match expression {
            Expression::Variable(_, replaced_expression) => {
                assert_eq!(replaced_var_expression, replaced_expression)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_expression_replace_functions_in_raw_expression() {
        let mut functions: HashMap<String, (Vec<String>, String)> = HashMap::new();

        functions.insert(
            String::from("distance"),
            (
                vec![String::from("x"), String::from("y")],
                String::from("x * x + y * y"),
            ),
        );

        functions.insert(
            String::from("f"),
            (vec![String::from("a")], String::from("a + 1")),
        );

        let raw_expression: String = String::from("distance(2.0, 3.3) + f(5.2) * 3");
        let replaced_raw_expression: String =
            String::from("(2.0 * 2.0 + 3.3 * 3.3) + (5.2 + 1) * 3");

        let mut expression: Expression = Expression::new(raw_expression.as_str());
        expression.replace_functions(&functions);

        match expression {
            Expression::Raw(replaced_expression) => {
                assert_eq!(replaced_raw_expression, replaced_expression)
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn test_expression_replace_functions_in_variable_expression() {
        let mut functions: HashMap<String, (Vec<String>, String)> = HashMap::new();

        functions.insert(
            String::from("distance"),
            (
                vec![String::from("x"), String::from("y")],
                String::from("x * x + y * y"),
            ),
        );

        functions.insert(
            String::from("f"),
            (vec![String::from("a")], String::from("a + 1")),
        );

        let var_expression: String = String::from("d = distance(2.0, 3.3) + f(5.2) * 3");
        let replaced_var_expression: String =
            String::from("(2.0 * 2.0 + 3.3 * 3.3) + (5.2 + 1) * 3");

        let mut expression: Expression = Expression::new(var_expression.as_str());
        expression.replace_functions(&functions);

        match expression {
            Expression::Variable(_, replaced_expression) => {
                assert_eq!(replaced_var_expression, replaced_expression)
            }
            _ => assert!(false),
        }
    }
}
