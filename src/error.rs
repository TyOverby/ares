use ::Value;
use ::parse::ParseError;

pub type AresResult<T> = Result<T, AresError>;

#[derive(Debug)]
pub enum AresError {
    ParseError(ParseError),
    NoProgram,

    UnexpectedType{value: Value, expected: String},
    UnexpectedArity{found: u16, expected: String},

    UnexecutableValue(Value),
    ExecuteEmptyList,

    UnexpectedArgsList(Value),

    IllegalConversion{value: Value, into: String},
    UndefinedName(String),
    InvalidState(String),

    // TODO: NoNamedSet, NoValuedSet
    NoNameSet,
    NoValueSet,

    InvalidForeignFunctionState,

    AlreadyDefined(String),
    NoNameDefine,
    NoValueDefine,
    MultiValueDefine,
}

impl From<ParseError> for AresError {
    fn from(pe: ParseError) -> AresError {
        AresError::ParseError(pe)
    }
}
