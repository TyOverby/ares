use std::rc::Rc;
use std::cell::RefCell;

use super::{Value, AresError, AresResult, Function};

pub use self::environment::{Env, Environment};
pub use self::foreign_function::{ForeignFunction, FfType};
pub use self::procedure::{Procedure, ParamBinding};

mod environment;
mod foreign_function;
mod procedure;

pub fn eval(value: &Value, env: &Rc<RefCell<Environment>>) -> AresResult<Value> {
    match value {
        &ref v@Value::String(_) => Ok(v.clone()),
        &ref v@Value::Int(_) => Ok(v.clone()),
        &ref v@Value::Float(_) => Ok(v.clone()),
        &ref v@Value::Bool(_) => Ok(v.clone()),

        &ref v@Value::LispFunction(_) => Ok(v.clone()),

        &Value::Ident(ref ident) => {
            match env.borrow().get(&ident) {
                Some(v) => Ok(v),
                None => Err(AresError::UndefinedName((**ident).clone()))
            }
        }

        &Value::List(ref l) => {
            let mut items = l.iter();
            let head = match items.next() {
                Some(h) => h,
                None => return Err(AresError::ExecuteEmptyList)
            };

            match try!(eval(head, env)) {
                Value::LispFunction(f) => {
                    if f.evaluate_arguments() {
                        let evald = try!(items.map(|v| eval(v, env)).collect::<AresResult<Vec<Value>>>());
                        apply(f, &mut evald.iter(), env)
                    } else {
                        apply(f, &mut items, env)
                    }
                },
                x => Err(AresError::UnexecutableValue(x))
            }
        }
    }
}
                    

pub fn apply<'a, I>(func: Function, args: &'a mut I, env: &'a Rc<RefCell<Environment>>) -> AresResult<Value> 
    where I: Iterator<Item=&'a Value>
{
    match func {
        Function::Lambda(procedure) => {
            let new_env = procedure.gen_env(args.cloned());
            let mut last = None;
            for body in &*procedure.bodies {
                last = Some(try!(eval(body, &new_env)));
            }
            last.ok_or(AresError::NoLambdaBody)
        },
        Function::ForeignFn(ff) => {
            match ff.function {
                FfType::FreeFn(ff) => (ff)(&mut args.cloned()),
                FfType::UnEvalFn(uef) => (uef)(args, env, &|v, e| eval(v, e))
            }
        }
    }
}
