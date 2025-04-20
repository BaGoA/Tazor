use std::collections::HashMap;

/// Kind of expression that we can parse
///
/// Raw expression is an expression that we want directly evaluate as `1 + 1`
///
/// Variable is an expression defining a variable that we want store.
/// It follows the template `variable_name = variable_definition`
///
/// ex: `x = 1 + 1`
///
/// Function is an expression defining a fucntion that we want store.
/// It follows the template `function_name: function_variable_1, function_variable_2, ... = function definition`
///
/// ex: `f: x, y = x * x + y * y`
///
pub enum Expression {
    Raw(String),
    Variable(String, String),
    Function(String, Vec<String>, String),
}

impl Expression {
    /// Construct an Expression from string
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
    ///
    /// The variables are given in argument through HashMap where
    /// pair (key, value) correspond respectively to name and value of variable
    pub fn replace_variables(&mut self, variables: &HashMap<String, f64>) {
        match self {
            Self::Raw(definition) | Self::Variable(_, definition) => {
                variables
                    .iter()
                    .for_each(|(variable_name, variable_value)| {
                        let mut replaced_definition: String = definition
                            .replace(variable_name, format!("{}", variable_value).as_str());

                        core::mem::swap(definition, &mut replaced_definition);
                    });
            }
            Self::Function(_, function_variables, definition) => {
                variables
                    .iter()
                    .filter(|(variable_name, _)| {
                        return !function_variables.contains(variable_name);
                    })
                    .for_each(|(variable_name, variable_value)| {
                        let mut replaced_definition: String = definition
                            .replace(variable_name, format!("{}", variable_value).as_str());

                        core::mem::swap(definition, &mut replaced_definition);
                    });
            }
        };
    }

    /// Recovery positions of function and its parenthesis in expression definition
    /// Expression definition and function name are given in argument
    fn get_function_positions(
        expression_definition: &String,
        fun_name: &String,
    ) -> Result<Option<(usize, usize, usize)>, String> {
        // Get position of function and its parenthesis
        let potential_start_position: Option<usize> = expression_definition.find(fun_name.as_str());

        if potential_start_position.is_none() {
            return Ok(None);
        }

        let start_position: usize = potential_start_position.unwrap();

        let start_search_parenthesis_position: usize = start_position + fun_name.len();

        let potential_opening_parenthesis_position: Option<usize> = expression_definition
            .chars()
            .skip(start_search_parenthesis_position)
            .position(|c| c == '(');

        if potential_opening_parenthesis_position.is_none() {
            return Ok(None);
        }

        let opening_parenthesis_position: usize =
            start_search_parenthesis_position + potential_opening_parenthesis_position.unwrap();

        let closing_parenthesis_position: usize = start_search_parenthesis_position
            + expression_definition
                .chars()
                .skip(start_search_parenthesis_position)
                .position(|c| c == ')')
                .ok_or(format!(
                    "Error occurs in call of function {}: Missing closing parenthesis",
                    fun_name
                ))?;

        // Check if we handle a function, else we go to next function name
        let has_char_between_fun_name_and_first_parenthesis: bool = expression_definition
            [start_search_parenthesis_position..opening_parenthesis_position]
            .chars()
            .any(|c| !c.is_whitespace());

        let has_opening_parenthesis_between_parenthesis: bool = expression_definition
            [(opening_parenthesis_position + 1)..closing_parenthesis_position]
            .chars()
            .any(|c| c == '(');

        if has_char_between_fun_name_and_first_parenthesis
            || has_opening_parenthesis_between_parenthesis
        {
            return Ok(None);
        }

        return Ok(Some((
            start_position,
            opening_parenthesis_position,
            closing_parenthesis_position,
        )));
    }

    /// Replace all function contained in expression by their definition
    ///
    /// The function are given in argument through HashMap where
    /// key correspond to name of function and value is a pair containing
    /// name of variables and definition of function
    pub fn replace_functions(
        &mut self,
        functions: &HashMap<String, (Vec<String>, String)>,
    ) -> Result<(), String> {
        let definition: &mut String = match self {
            Self::Raw(raw_expression) => raw_expression,
            Self::Variable(_, definition) => definition,
            Self::Function(_, _, definition) => definition,
        };

        for fun_name in functions.keys() {
            // Get positions of function name and its parenthesis
            let potential_positions: Option<(usize, usize, usize)> =
                Expression::get_function_positions(&definition, fun_name)?;

            if potential_positions.is_none() {
                // here the functions is not in expression definition
                continue;
            }

            let (start_position, opening_parenthesis_position, closing_parenthesis_position) =
                potential_positions.unwrap();

            // Get value of function variables
            let variable_values: Vec<&str> = definition
                [(opening_parenthesis_position + 1)..closing_parenthesis_position]
                .split(", ")
                .collect();

            // Create string to replace function call by function body
            let variables: &Vec<String> = functions[fun_name].0.as_ref();
            let mut replaced_fun_definition: String = functions[fun_name].1.clone();

            if variables.len() != variable_values.len() {
                return Err(format!("The number of variables is not consistent"));
            }

            let mut id: usize = 0;

            for variable in variables {
                replaced_fun_definition =
                    replaced_fun_definition.replace(variable, variable_values[id]);

                id += 1;
            }

            definition.replace_range(
                start_position..=closing_parenthesis_position,
                format!("({})", replaced_fun_definition).as_str(),
            );
        }

        return Ok(());
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
        expression.replace_functions(&functions).unwrap();

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
        expression.replace_functions(&functions).unwrap();

        match expression {
            Expression::Variable(_, replaced_expression) => {
                assert_eq!(replaced_var_expression, replaced_expression)
            }
            _ => assert!(false),
        }
    }
}
