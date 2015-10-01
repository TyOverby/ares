use ::Value;
use ::parse::ParseError;

pub type AresResult<T> = Result<T, AresError>;

#[derive(Debug)]
pub enum AresError {
    ParseError(ParseError),

    UnexpectedType{value: Value, expected: String},
    UnexpectedArity{found: u16, expected: String},

    UnexecutableValue(Value),
    ExecuteEmptyList,

    UnexpectedArgsList(Value),

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

