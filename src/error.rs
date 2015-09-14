use ::Value;

pub type AresResult<T> = Result<T, AresError>;

#[derive(Debug)]
pub enum AresError {
    UnexpectedType{value: Value, expected: String},
    UndefinedName(String),
    UnexecutableValue(Value),
    ExecuteEmptyList,
    NoLambdaBody
    //UnexpectedArity{expected: u16, found: u16}
}

