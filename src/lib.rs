/// Trait to define mathematical expression evaluator
pub trait Evaluator {
    /// Evaluate methematical expression given in argument
    /// If error occurs during evaluation, an error message is stored in string contained in Result output.
    /// Otherwise, the Result output contains the value of evaluation stored in 64-bits float.
    fn evaluate(expression: &String) -> Result<f64, String>;
}
