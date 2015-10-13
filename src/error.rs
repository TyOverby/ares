use std::any::Any;

use ::Value;
use ::parse::ParseError;

pub type AresResult<T> = Result<T, AresError>;

// TODO: this should derive Eq
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
    InvalidUnquotation,

    // TODO: NoNamedSet, NoValuedSet
    NoNameSet,
    NoValueSet,

    InvalidForeignFunctionState,
    AstFunctionPass,

    AlreadyDefined(String),
    NoNameDefine,
    NoValueDefine,
    MultiValueDefine,

    UserError(Box<Any>)
}

impl AresError {
    pub fn user_error<T: Any>(t: T) -> AresError {
        AresError::UserError(Box::new(t) as Box<Any>)
    }
}

impl From<ParseError> for AresError {
    fn from(pe: ParseError) -> AresError {
        AresError::ParseError(pe)
    }
}
