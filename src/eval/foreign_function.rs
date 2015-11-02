use std::rc::Rc;
use std::any::TypeId;

use {Value, AresResult, rc_to_usize, write_usize, State};

use super::context::LoadedContext;

#[derive(Clone, Eq, PartialEq)]
pub enum FfType {
    Free,
    User,
    Ast,
}

#[derive(Clone)]
pub struct ForeignFunction<S: State + ?Sized> {
    pub name: String,
    pub typ: FfType,
    #[doc(hidden)]
    pub function: Rc<Fn(&[Value], &mut LoadedContext<S>) -> AresResult<Value>>,
    typeid: TypeId,
}

impl <S: State + ?Sized> ForeignFunction<S> {
    pub fn erase(self) -> ForeignFunction<()> {
        use std::mem::transmute;
        unsafe { transmute(self) }
    }
}

impl ForeignFunction<()> {
    pub fn correct<S: State + ?Sized>(self) -> Result<ForeignFunction<S>, ForeignFunction<()>> {
        use std::mem::transmute;
        if TypeId::of::<S>() == self.typeid {
            Ok(unsafe { transmute(self) })
        } else {
            Err(self)
        }
    }
}

pub fn free_fn<S: State + ?Sized, N, F>(name: N, func: F) -> ForeignFunction<S>
    where N: Into<String>,
          F: Fn(&[Value]) -> AresResult<Value> + 'static
{
    let closure = move |values: &[Value], ctx: &mut LoadedContext<S>| {
        let evaluated: Result<Vec<_>, _> = values.iter().map(|v| ctx.eval(v)).collect();
        let evaluated = try!(evaluated);
        func(&evaluated[..])
    };

    let boxed = Rc::new(closure);
    ForeignFunction {
        name: name.into(),
        function: boxed,
        typeid: TypeId::of::<S>(),
        typ: FfType::Free,
    }
}

pub fn user_fn<S: State + ?Sized, N, F>(name: N, func: F) -> ForeignFunction<S>
    where N: Into<String>,
          F: Fn(&[Value], &mut LoadedContext<S>) -> AresResult<Value> + 'static
{
    let closure = move |values: &[Value], ctx: &mut LoadedContext<S>| {
        let evaluated: Result<Vec<_>, _> = values.iter().map(|v| ctx.eval(v)).collect();
        let evaluated = try!(evaluated);
        func(&evaluated[..], ctx)
    };

    let boxed = Rc::new(closure);
    ForeignFunction {
        name: name.into(),
        function: boxed,
        typeid: TypeId::of::<S>(),
        typ: FfType::User,
    }
}


pub fn ast_fn<S: State + ?Sized, N, F>(name: N, func: F) -> ForeignFunction<S>
    where N: Into<String>,
          F: Fn(&[Value], &mut LoadedContext<S>) -> AresResult<Value> + 'static
{
    let boxed = Rc::new(func);
    ForeignFunction {
        name: name.into(),
        function: boxed,
        typeid: TypeId::of::<S>(),
        typ: FfType::Ast,
    }
}


impl <S: State + ?Sized> ::std::fmt::Debug for ForeignFunction<S> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        fmt.write_str(&self.name)
    }
}

impl <S: State + ?Sized> PartialEq for ForeignFunction<S> {
    fn eq(&self, other: &ForeignFunction<S>) -> bool {
        self.name == other.name && rc_to_usize(&self.function) == rc_to_usize(&other.function)
    }
}

impl <S: State + ?Sized> Eq for ForeignFunction<S> {}

impl <S: State + ?Sized> ::std::hash::Hash for ForeignFunction<S> {
    fn hash<H>(&self, state: &mut H)
        where H: ::std::hash::Hasher
    {
        write_usize(rc_to_usize(&self.function), state);
    }
}
