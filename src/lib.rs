#![allow(dead_code)]
use std::collections::HashMap;

pub struct Calculator<Evaluator>
where
    Evaluator: Fn(&str) -> Result<f64, String>,
{
    evaluator: Evaluator,               // mathematical expression evaluator
    variables: HashMap<String, String>, // map to store custom variable defined by user, key is name of variable and value is its evaluation
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
    pub fn process(&mut self, expression: &str) -> Result<(String, String), String> {
        let variable_name: String = String::from("last");
        let variable_value: String = format!("{}", (self.evaluator)(expression)?);

        self.variables
            .insert(variable_name.clone(), variable_value.clone());

        return Ok((variable_name, variable_value));
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

                let variable_value: String = format!("{}", expression.len());
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

                let variable_value: String = format!("{}", first_expression.len());
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

                let variable_value: String = format!("{}", second_expression.len());
                assert_eq!(value, variable_value);

                assert_eq!(calculator.variables.len(), 1);
                assert!(calculator.variables.contains_key(&variable_name));
                assert_eq!(calculator.variables[&variable_name], variable_value);
            }
            Err(_) => assert!(false),
        }
    }
}
