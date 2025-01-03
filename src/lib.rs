#![allow(dead_code)]
pub mod expression;

use expression::Expression;

use std::collections::HashMap;

pub struct Calculator<Evaluator>
where
    Evaluator: Fn(&str) -> Result<f64, String>,
{
    evaluator: Evaluator,            // mathematical expression evaluator
    variables: HashMap<String, f64>, // map to store custom variable defined by user, key is name of variable and value is its evaluation
}

impl<Evaluator> Calculator<Evaluator>
where
    Evaluator: Fn(&str) -> Result<f64, String>,
{
    /// Construct a calculator given an evaluator in argument
    pub fn new(evaluator: Evaluator) -> Self {
        return Self {
            evaluator,
            variables: HashMap::with_capacity(25),
        };
    }

    /// Process an expression
    /// If error occurs during process, an error message is stored in string contained in Result output.
    /// Otherwise, the Result output contains the (key, value) storage in variable map, corresponding to name and value
    /// of evaluated expression
    pub fn process(&mut self, expression_str: &str) -> Result<(String, f64), String> {
        // Create expression and replace all variable it contains
        let mut expression: Expression = Expression::new(expression_str);
        expression.replace_variables(&self.variables);

        // Evaluate the expression and store it in variable hash map
        let (name, value): (String, f64) = match expression {
            Expression::Raw(raw_expression) => (
                String::from("last"),
                (self.evaluator)(raw_expression.as_str())?,
            ),
            Expression::Variable(name, definition) => {
                (name, (self.evaluator)(definition.as_str())?)
            }
        };

        self.variables.insert(name.clone(), value);

        return Ok((name, value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Define mock evaluator for units tests
    fn evaluate(expression: &str) -> Result<f64, String> {
        if expression.is_empty() {
            return Err(String::from("Expression is empty"));
        }

        return Ok(expression.len() as f64);
    }

    #[test]
    fn test_calculator_new() {
        let calculator = Calculator::new(evaluate);

        assert!(calculator.variables.capacity() > 0);

        let empty_expression: String = String::default();
        assert!((calculator.evaluator)(empty_expression.as_str()).is_err());

        let expression: String = String::from("taz");
        let result: Result<f64, String> = (calculator.evaluator)(expression.as_str());

        assert!(result.is_ok());
        assert_eq!(result.unwrap() as usize, expression.len())
    }

    #[test]
    fn test_calculator_process_raw_expression() {
        let mut calculator = Calculator::new(evaluate);

        let expression: String = String::from("1 + 1");

        match calculator.process(expression.as_str()) {
            Ok((name, value)) => {
                let variable_name: String = String::from("last");
                assert_eq!(name, variable_name);

                let variable_value: f64 = expression.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&variable_name));
                assert_eq!(calculator.variables[&variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_several_raw_expression() {
        let mut calculator = Calculator::new(evaluate);

        // Evaluate first expression
        let first_expression: String = String::from("1 + 1");

        let variable_name: String = String::from("last");

        match calculator.process(first_expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, variable_name);

                let variable_value: f64 = first_expression.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&variable_name));
                assert_eq!(calculator.variables[&variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }

        // Evaluate second expresion
        // The value of variable named 'last' must be replaced by value of second expression
        let second_expression: String = String::from("1 + 1 + 3");

        match calculator.process(second_expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, variable_name);

                let variable_value: f64 = second_expression.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&variable_name));
                assert_eq!(calculator.variables[&variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_variable_expression() {
        let mut calculator = Calculator::new(evaluate);

        let variable_name: String = String::from("x");
        let variable_definition: String = String::from("1 + 1");

        let expression: String = format!("{} = {}", variable_name, variable_definition);

        match calculator.process(expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, variable_name);

                let variable_value: f64 = variable_definition.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&variable_name));
                assert_eq!(calculator.variables[&variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_several_variable_expression() {
        let mut calculator = Calculator::new(evaluate);

        let first_variable_name: String = String::from("x");
        let first_variable_definition: String = String::from("1 + 1");

        let first_expression: String =
            format!("{} = {}", first_variable_name, first_variable_definition);

        match calculator.process(first_expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, first_variable_name);

                let variable_value: f64 = first_variable_definition.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&first_variable_name));
                assert_eq!(calculator.variables[&first_variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }

        let second_variable_name: String = String::from("y");
        let second_variable_definition: String = String::from("9 + 1");

        let second_expression: String =
            format!("{} = {}", second_variable_name, second_variable_definition);

        match calculator.process(second_expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, second_variable_name);

                let variable_value: f64 = second_variable_definition.len() as f64;
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 2);
                assert!(calculator.variables.contains_key(&second_variable_name));
                assert_eq!(calculator.variables[&second_variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_expression_with_variables() {
        let mut calculator = Calculator::new(evaluate);

        let first_variable_name: String = String::from("x");
        let first_variable_definition: String = String::from("1 + 1");

        let first_expression: String =
            format!("{} = {}", first_variable_name, first_variable_definition);

        let first_process_result = calculator.process(first_expression.as_str());
        assert!(first_process_result.is_ok());

        let second_variable_name: String = String::from("y");
        let second_variable_definition: String = String::from("97 + 1");

        let second_expression: String =
            format!("{} = {}", second_variable_name, second_variable_definition);

        let second_process_result = calculator.process(second_expression.as_str());
        assert!(second_process_result.is_ok());

        let variable_name: String = String::from("distance");
        let variable_definition: String = String::from("x + y");

        let expression: String = format!("{} = {}", variable_name, variable_definition);

        match calculator.process(expression.as_str()) {
            Ok((name, value)) => {
                assert_eq!(name, variable_name);

                let variable_value: f64 = (first_variable_definition.len().to_string().len()
                    + second_variable_definition.len().to_string().len()
                    + 3) as f64;

                assert_eq!(value, variable_value);
            }
            Err(_) => assert!(false),
        }
    }
}
