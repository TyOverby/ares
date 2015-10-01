use ::{Value, AresResult, AresError, free_fn, LoadedContext};
use super::util::expect_arity;

pub fn build_list(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
    use std::rc::Rc;
    use std::cell::RefCell;

    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    let vec = Rc::new(RefCell::new(Some(Vec::<Value>::new())));
    let writer = vec.clone();

    let func = move |values: &[Value]| -> AresResult<Value> {
        match &mut *writer.borrow_mut() {
            &mut Some(ref mut adder) => {
                let mut last = None;
                for value in values {
                    adder.push(value.clone());
                    last = Some(value);
                }

                match last {
                    Some(v) => Ok(v.clone()),
                    None => Err(AresError::UnexpectedArity {
                        found: 0,
                        expected: "at least 1".to_string()
                    })
                }
            },
            &mut None => {
                let err_msg = "build-list adder called after completion of build-list.";
                return Err(AresError::InvalidState(err_msg.to_string()))
           }
        }
    };

    let boxed_fn: Value = free_fn("add", func);

    let evaluator = args[0].clone();
    // TODO: should this be apply?
    try!(ctx.eval(&Value::new_list(vec![evaluator, boxed_fn])));

    let mut v = vec.borrow_mut();
    Ok(Value::new_list(v.take().unwrap()))
}

pub fn foreach(args: &[Value], ctx: &mut LoadedContext) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let should_be_list = args[0].clone();
    let list: Vec<_> = match try!(ctx.eval(&should_be_list)) {
        Value::List(ref l) => (&**l).clone(),
        other => return Err(AresError::UnexpectedType{
            value: other,
            expected: "List".into()
        }),
    };

    let func = args[1].clone();
    let func = try!(ctx.eval(&func));

    let mut count = 0;
    for element in list {
        let singleton_slice: [Value; 1] = [element];
        try!(ctx.call(&func, &singleton_slice[..]));
        count += 1;
    }

    Ok(Value::Int(count))
}

pub static LIST: &'static str = "(lambda list list)";

pub static MAP: &'static str =
"(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (push (fn element)))))))";

pub static FOLD_LEFT: &'static str =
"(lambda (list default fn)
    (for-each list (lambda (element)
        (set default (fn default element))
    ))
    default)";

pub static FILTER: &'static str =
"(lambda (list fn)
    (build-list
        (lambda (push)
            (for-each list (lambda (element)
                (if (fn element)
                    (push element)
                    false))))))";

pub static CONCAT: &'static str =
"(lambda lists
    (build-list
        (lambda (push)
            (for-each lists (lambda (list)
                (for-each list (lambda (element)
                    (push element))))))))";
