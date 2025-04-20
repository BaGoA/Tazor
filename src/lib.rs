//! Tazor
//!
//! Tazor is Rust library implementing a calculator based on mathematical expression
//!

pub mod expression;

use expression::Expression;

use std::collections::HashMap;

/// Evaluate mathematical expression and store user-define variable and function to reuse it after.
///
/// The calculator is based on Evaluator which is a function taking a string, representing a mathematical expression
/// and return the value of evaluation on 64-bit float.
///
/// # Example with simple expression
/// ```
/// use tazor;
///
/// fn evaluate(expression: &str) -> Result<f64, String> {
///     if expression.is_empty() {
///         return Err(String::from("Expression is empty"));
///     }
///
///     return Ok(expression.len() as f64);
/// }
///
/// let mut calculator = tazor::Calculator::new(evaluate);
///
/// let expression: String = String::from("1 + 1");
///
/// match calculator.process(expression.as_str()) {
///     Ok(str_result) => println!("{}", str_result),
///     Err(_) => assert!(false),
/// }
///
/// ```
///
/// # Example with expression defining variables and use it in other expression
/// ```
/// use tazor;
///
/// fn evaluate(expression: &str) -> Result<f64, String> {
///     if expression.is_empty() {
///         return Err(String::from("Expression is empty"));
///     }
///
///     return Ok(expression.len() as f64);
/// }
///
/// let mut calculator = tazor::Calculator::new(evaluate);
///
/// let variable_x: String = String::from("x = 4.5 * 23.67");
/// assert!(calculator.process(variable_x.as_str()).is_ok());
///
/// let variable_y: String = String::from("y = 43.5 + 2.75");
/// assert!(calculator.process(variable_y.as_str()).is_ok());
///
/// let expression: String = String::from("x * x + y * y");
///
/// match calculator.process(expression.as_str()) {
///     Ok(str_result) => println!("{}", str_result),
///     Err(_) => assert!(false),
/// }
///
/// ```
///
/// # Example with expression defining a variable, a function and use it in other expression
/// ```
/// use tazor;
///
/// fn evaluate(expression: &str) -> Result<f64, String> {
///     if expression.is_empty() {
///         return Err(String::from("Expression is empty"));
///     }
///
///     return Ok(expression.len() as f64);
/// }
///
/// let mut calculator = tazor::Calculator::new(evaluate);
///
/// let factor: String = String::from("factor = 4.5 * 23.67");
/// assert!(calculator.process(factor.as_str()).is_ok());
///
/// let function: String = String::from("f: x, y = factor * (x * x + y * y)");
/// assert!(calculator.process(function.as_str()).is_ok());
///
/// let expression: String = String::from("f(1,75, 2.54) + 2.43");
///
/// match calculator.process(expression.as_str()) {
///     Ok(str_result) => println!("{}", str_result),
///     Err(_) => assert!(false),
/// }
///
/// ```
///
pub struct Calculator<Evaluator>
where
    Evaluator: Fn(&str) -> Result<f64, String>,
{
    evaluator: Evaluator,            // mathematical expression evaluator
    variables: HashMap<String, f64>, // map to store custom variable defined by user, key is name of variable and value is its evaluation
    functions: HashMap<String, (Vec<String>, String)>, // map to store custom function defined by user, key is name of function and value is its expression (variables, definition)
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
            functions: HashMap::with_capacity(25),
        };
    }

    /// Process an expression
    ///
    /// If error occurs during process, an error message is stored in string contained in Result output.
    ///
    /// Otherwise, the Result output contains string which represent result according to kind of expression:
    ///    - raw => `last = evaluated_expression`
    ///    - variable => `variable_name = variable_value`
    ///    - function => `function_name(function_variables) = function_definition`
    ///
    pub fn process(&mut self, expression_str: &str) -> Result<String, String> {
        let mut expression: Expression = Expression::new(expression_str);

        expression.replace_functions(&self.functions)?;
        expression.replace_variables(&self.variables);

        let result: String = match expression {
            Expression::Raw(raw_expression) => {
                let value: f64 = (self.evaluator)(&raw_expression.as_str())?;

                let raw_expression_result: String = format!("last = {}", value);
                self.variables.insert(String::from("last"), value);

                raw_expression_result
            }
            Expression::Variable(name, definition) => {
                let value: f64 = (self.evaluator)(&definition.as_str())?;

                let variable_result: String = format!("{} = {}", name, value);
                self.variables.insert(name, value);

                variable_result
            }
            Expression::Function(name, variables, definition) => {
                let function_result: String =
                    format!("{}({}) = {}", name, variables.join(", "), definition);

                self.functions.insert(name, (variables, definition));

                function_result
            }
        };

        return Ok(result);
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
            Ok(str_result) => {
                let variable_name: String = String::from("last");
                let variable_value: f64 = expression.len() as f64;

                let str_reference: String = format!("{} = {}", variable_name, variable_value);
                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = first_expression.len() as f64;

                let str_reference: String = format!("{} = {}", variable_name, variable_value);
                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = second_expression.len() as f64;

                let str_reference: String = format!("{} = {}", variable_name, variable_value);
                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = variable_definition.len() as f64;

                let str_reference: String = format!("{} = {}", variable_name, variable_value);
                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = first_variable_definition.len() as f64;

                let str_reference: String = format!("{} = {}", first_variable_name, variable_value);
                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = second_variable_definition.len() as f64;

                let str_reference: String =
                    format!("{} = {}", second_variable_name, variable_value);

                assert_eq!(str_result, str_reference);

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
            Ok(str_result) => {
                let variable_value: f64 = (first_variable_definition.len().to_string().len()
                    + second_variable_definition.len().to_string().len()
                    + 3) as f64;

                let str_reference: String = format!("{} = {}", variable_name, variable_value);
                assert_eq!(str_result, str_reference);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_function_expression() {
        let mut calculator = Calculator::new(evaluate);

        let function_name: String = String::from("distance");
        let function_variables: Vec<String> = vec![String::from("x"), String::from("y")];
        let function_definition: String = format!(
            "{} * {} + {} * {}",
            function_variables[0],
            function_variables[0],
            function_variables[1],
            function_variables[1]
        );

        let expression: String = format!(
            "{}: {}, {} = {}",
            function_name, function_variables[0], function_variables[1], function_definition
        );

        match calculator.process(expression.as_str()) {
            Ok(str_result) => {
                let str_reference: String = format!(
                    "{}({}) = {}",
                    function_name,
                    function_variables.join(", "),
                    function_definition
                );

                assert_eq!(str_result, str_reference);

                assert_eq!(calculator.functions.len(), 1);
                assert!(calculator.functions.contains_key(&function_name));
                assert_eq!(
                    calculator.functions[&function_name],
                    (function_variables, function_definition)
                );
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_several_function_expression() {
        let mut calculator = Calculator::new(evaluate);

        let first_function_name: String = String::from("distance");
        let first_function_variables: Vec<String> = vec![String::from("x"), String::from("y")];
        let first_function_definition: String = format!(
            "{} * {} + {} * {}",
            first_function_variables[0],
            first_function_variables[0],
            first_function_variables[1],
            first_function_variables[1]
        );

        let first_expression: String = format!(
            "{}: {}, {} = {}",
            first_function_name,
            first_function_variables[0],
            first_function_variables[1],
            first_function_definition
        );

        match calculator.process(first_expression.as_str()) {
            Ok(str_result) => {
                let str_reference: String = format!(
                    "{}({}) = {}",
                    first_function_name,
                    first_function_variables.join(", "),
                    first_function_definition
                );

                assert_eq!(str_result, str_reference);

                assert_eq!(calculator.functions.len(), 1);
                assert!(calculator.functions.contains_key(&first_function_name));
                assert_eq!(
                    calculator.functions[&first_function_name],
                    (first_function_variables, first_function_definition)
                );
            }
            Err(_) => assert!(false),
        }

        let second_function_name: String = String::from("velocity");
        let second_function_variables: Vec<String> =
            vec![String::from("distance"), String::from("time")];

        let second_function_definition: String = format!(
            "{} / {}",
            second_function_variables[0], second_function_variables[1]
        );

        let second_expression: String = format!(
            "{}: {}, {} = {}",
            second_function_name,
            second_function_variables[0],
            second_function_variables[1],
            second_function_definition
        );

        match calculator.process(second_expression.as_str()) {
            Ok(str_result) => {
                let str_reference: String = format!(
                    "{}({}) = {}",
                    second_function_name,
                    second_function_variables.join(", "),
                    second_function_definition
                );

                assert_eq!(str_result, str_reference);

                assert_eq!(calculator.functions.len(), 2);
                assert!(calculator.functions.contains_key(&second_function_name));
                assert_eq!(
                    calculator.functions[&second_function_name],
                    (second_function_variables, second_function_definition)
                );
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_expression_with_functions() {
        let mut calculator = Calculator::new(evaluate);

        let first_function_name: String = String::from("distance");
        let first_function_variables: Vec<String> = vec![String::from("x"), String::from("y")];
        let first_function_definition: String = format!(
            "{} * {} + {} * {}",
            first_function_variables[0],
            first_function_variables[0],
            first_function_variables[1],
            first_function_variables[1]
        );

        let first_function_expression: String = format!(
            "{}: {}, {} = {}",
            first_function_name,
            first_function_variables[0],
            first_function_variables[1],
            first_function_definition
        );

        assert!(calculator
            .process(first_function_expression.as_str())
            .is_ok());

        let second_function_name: String = String::from("velocity");
        let second_function_variables: Vec<String> =
            vec![String::from("distance"), String::from("time")];

        let second_function_definition: String = format!(
            "{} / {}",
            second_function_variables[0], second_function_variables[1]
        );

        let second_function_expression: String = format!(
            "{}: {}, {} = {}",
            second_function_name,
            second_function_variables[0],
            second_function_variables[1],
            second_function_definition
        );

        assert!(calculator
            .process(second_function_expression.as_str())
            .is_ok());

        let expression: String = format!(
            "3.14 * {}(6.89, 5.43) - {}(2.4, 4.3) + (2 * 3 - 7)",
            second_function_name, first_function_name
        );

        let replaced_expression: String =
            String::from("3.14 * (6.89 / 5.43) - (2.4 * 2.4 + 4.3 * 4.3) + (2 * 3 - 7)");

        match calculator.process(expression.as_str()) {
            Ok(str_result) => {
                let str_reference: String = format!("last = {}", replaced_expression.len());
                assert_eq!(str_result, str_reference);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_expression_with_variables_and_functions() {
        let mut calculator = Calculator::new(evaluate);

        let first_function_name: String = String::from("distance");
        let first_function_variables: Vec<String> = vec![String::from("x"), String::from("y")];
        let first_function_definition: String = format!(
            "{} * {} + {} * {}",
            first_function_variables[0],
            first_function_variables[0],
            first_function_variables[1],
            first_function_variables[1]
        );

        let first_function_expression: String = format!(
            "{}: {}, {} = {}",
            first_function_name,
            first_function_variables[0],
            first_function_variables[1],
            first_function_definition
        );

        assert!(calculator
            .process(first_function_expression.as_str())
            .is_ok());

        let second_function_name: String = String::from("velocity");
        let second_function_variables: Vec<String> =
            vec![String::from("distance"), String::from("time")];

        let second_function_definition: String = format!(
            "{} / {}",
            second_function_variables[0], second_function_variables[1]
        );

        let second_function_expression: String = format!(
            "{}: {}, {} = {}",
            second_function_name,
            second_function_variables[0],
            second_function_variables[1],
            second_function_definition
        );

        assert!(calculator
            .process(second_function_expression.as_str())
            .is_ok());

        let first_variable_name: String = String::from("x");
        let first_variable_definition: String = String::from("1 + 1");

        let first_expression: String =
            format!("{} = {}", first_variable_name, first_variable_definition);

        assert!(calculator.process(first_expression.as_str()).is_ok());

        let second_variable_name: String = String::from("y");
        let second_variable_definition: String = String::from("97 + 1");

        let second_expression: String =
            format!("{} = {}", second_variable_name, second_variable_definition);

        assert!(calculator.process(second_expression.as_str()).is_ok());

        let expression: String = format!(
            "3.14 * {}({}, {}) - {}(2.4, 4.3) + (2 * 3 - 7)",
            second_function_name, first_variable_name, second_variable_name, first_function_name
        );

        let replaced_expression: String = format!(
            "3.14 * ({} / {}) - (2.4 * 2.4 + 4.3 * 4.3) + (2 * 3 - 7)",
            first_variable_definition.len(),
            second_variable_definition.len()
        );

        match calculator.process(expression.as_str()) {
            Ok(str_result) => {
                let str_reference: String = format!("last = {}", replaced_expression.len());
                assert_eq!(str_result, str_reference);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_calculator_process_expression_with_variables_and_function_variables_equal_to_variables()
    {
        let mut calculator = Calculator::new(evaluate);

        let first_variable_name: String = String::from("x");
        let first_variable_definition: String = String::from("1 + 1");

        let first_expression: String =
            format!("{} = {}", first_variable_name, first_variable_definition);

        assert!(calculator.process(first_expression.as_str()).is_ok());

        let second_variable_name: String = String::from("y");
        let second_variables_definition: String = String::from("2 * 5");

        let second_expression: String =
            format!("{} = {}", second_variable_name, second_variables_definition);

        assert!(calculator.process(second_expression.as_str()).is_ok());

        let function_name: String = String::from("distance");
        let function_variables: Vec<String> = vec![String::from("x"), String::from("y")];
        let function_definition: String = format!(
            "{} * {} + {} * {}",
            function_variables[0],
            function_variables[0],
            function_variables[1],
            function_variables[1]
        );

        let function_expression: String = format!(
            "{}: {}, {} = {}",
            function_name, function_variables[0], function_variables[1], function_definition
        );

        assert!(calculator.process(function_expression.as_str()).is_ok());
        assert!(function_definition == calculator.functions[&function_name].1);
    }
}
