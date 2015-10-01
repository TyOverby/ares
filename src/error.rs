use ::Value;
use ::tokenizer::ParseError;

pub type AresResult<T, S> = Result<T, AresError<S>>;

pub enum AresError<S> {
    ParseError(ParseError),
    NoProgram,

    UnexpectedType{value: Value<S>, expected: String},
    UnexpectedArity{found: u16, expected: String},

    UnexecutableValue(Value<S>),
    ExecuteEmptyList,

    UnexpectedArgsList(Value<S>),

    IllegalConversion{value: Value<S>, into: String},
    UndefinedName(String),
    InvalidState(String),

    // TODO: NoNamedSet, NoValuedSet
    NoNameSet,
    NoValueSet,


    AlreadyDefined(String),
    NoNameDefine,
    NoValueDefine,
    MultiValueDefine,
}

impl <S> From<ParseError> for AresError<S> {
    fn from(pe: ParseError) -> AresError<S> {
        AresError::ParseError(pe)
    }
}


use std::fmt::{Debug, Formatter, Error};
impl <S> Debug for AresError<S> {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str("todo lol")
    }
}
