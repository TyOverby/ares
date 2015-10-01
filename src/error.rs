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
        match self {
            &AresError::ParseError(ref pe) => write!(fmt, "ParseError({})", pe),
            &AresError::NoProgram => write!(fmt, "NoProgram"),

            &AresError::UnexpectedType{ref value, ref expected} =>
                write!(fmt, "UnexpectedType(found {:?}, expected {})", value, expected),
            &AresError::UnexpectedArity{ref found, ref expected} =>
                write!(fmt, "UnexpectedArity(found {}, expected {})", found, expected),

            &AresError::UnexecutableValue(ref v) =>
                write!(fmt, "UnexecutableValue({:?})", v),
            &AresError::ExecuteEmptyList =>
                write!(fmt, "ExecuteEmptyList"),

            &AresError::UnexpectedArgsList(ref v) =>
                write!(fmt, "UnexpectedArgsList({:?})", v),

            &AresError::IllegalConversion{ref value, ref into} =>
                write!(fmt, "IllegalConversion(from {:?}, into {})", value, into),
            &AresError::UndefinedName(ref name) =>
                write!(fmt, "UndefinedName({})", name),
            &AresError::InvalidState(ref desc) =>
                write!(fmt, "InvalidState({})", desc),

            // TODO: NoNamedSet, NoValuedSet
            &AresError::NoNameSet => fmt.write_str("NoNameSet"),
            &AresError::NoValueSet => fmt.write_str("NoValueSet"),

            &AresError::AlreadyDefined(ref name) =>
                write!(fmt, "AlreadyDefined({})", name),
            &AresError::NoNameDefine => fmt.write_str("NoNameDefine"),
            &AresError::NoValueDefine => fmt.write_str("NoValueDefine"),
            &AresError::MultiValueDefine => fmt.write_str("MultiValueDefine"),

        }
    }
}
