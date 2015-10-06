use ::{Value, AresResult, AresError, free_fn, LoadedContext, State};
use super::util::expect_arity;

pub fn build_list<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    use std::rc::Rc;
    use std::cell::RefCell;

    try!(expect_arity(args, |l| l == 1, "exactly 1"));
    let vec = Rc::new(RefCell::new(Some(Vec::<Value>::new())));
    let writer1 = vec.clone();
    let writer2 = vec.clone();

    let push_individuals = move |values: &[Value]| -> AresResult<Value> {
        try!(expect_arity(values, |l| l >= 1, "at least 1"));

        match &mut *writer1.borrow_mut() {
            &mut Some(ref mut adder) => {
                let mut last = None;
                for value in values {
                    adder.push(value.clone());
                    last = Some(value);
                }

                Ok(last.unwrap().clone()) // safe because of the expect_arity
            },
            &mut None => {
                let err_msg = "build-list `add`er called after completion of build-list.";
                return Err(AresError::InvalidState(err_msg.to_string()))
           }
        }
    };

    let push_list_values = move |values: &[Value]| -> AresResult<Value> {
        try!(expect_arity(values, |l| l >= 1, "at least 1"));

        match &mut *writer2.borrow_mut() {
            &mut Some(ref mut adder) => {
                let mut last = None;
                for value in values {
                    if let &Value::List(ref list) = value {
                        for element in &***list {
                            adder.push(element.clone());
                        }
                    }
                    last = Some(value);
                }

                Ok(last.unwrap().clone()) // safe because of the expect_arity
            },
            &mut None => {
                let err_msg = "build-list `add`er called after completion of build-list.";
                return Err(AresError::InvalidState(err_msg.to_string()))
           }
        }
    };

    let boxed_push_indiv: Value = Value::ForeignFn(free_fn::<S, _, _>("add", push_individuals).erase());
    let boxed_push_list: Value = Value::ForeignFn(free_fn::<S, _, _>("add-all", push_list_values).erase());

    let evaluator = args[0].clone();
    // TODO: should this be apply?
    try!(ctx.eval(&Value::list(vec![evaluator, boxed_push_indiv, boxed_push_list])));

    let mut v = vec.borrow_mut();
    Ok(Value::list(v.take().unwrap()))
}

pub fn foreach<S: State + ?Sized>(args: &[Value], ctx: &mut LoadedContext<S>) -> AresResult<Value> {
    try!(expect_arity(args, |l| l == 2, "exactly 2"));
    let list: Vec<_> = match args[0] {
        Value::List(ref l) => (&**l).clone(),
        ref other => return Err(AresError::UnexpectedType{
            value: other.clone(),
            expected: "List".into()
        })
    };

    let ref func = args[1];

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

pub static FLATTEN: &'static str =
"(lambda (list-of-lists)
    (build-list
        (lambda (push push-all)
            (for-each list-of-lists (lambda (sub-list)
                (push-all sub-list))))))";

pub static CONCAT: &'static str =
"(lambda lists
    (flatten lists))";

