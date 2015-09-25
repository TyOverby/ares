use ::Value;
use ::tokenizer::ParseError;

pub type AresResult<T> = Result<T, AresError>;

#[derive(Debug)]
pub enum AresError {
    ParseError(ParseError),

    UnexpectedType{value: Value, expected: String},
    UnexpectedArity{found: u16, expected: String},

    UnexecutableValue(Value),
    ExecuteEmptyList,

    NoLambdaArgsList,
    UnexpectedArgsList(Value),
    NoLambdaBody,

    IllegalConversion{value: Value, into: String},
    UndefinedName(String),
    InvalidState(String),

    NoNameSet,
    NoValueSet,


    AlreadyDefined(String),
    NoNameDefine,
    NoValueDefine,
    MultiValueDefine,
}

