mod integration_tests {
    use crate::buffer::Buffer;
    use crate::evaluator::eval_all;
    use crate::parser::parse_all;
    use crate::types::rational::Rational;
    use crate::types::Exp;
    use crate::{lib::SchemeError, types::Value};
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::environment::Env;

    fn evaluate_input(input: &str) -> Result<Exp, SchemeError> {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let mut buffer = Buffer::from(input);
        let exp = parse_all(&mut buffer)?;
        eval_all(&exp, &mut env)
    }

    fn evaluate_input_with_env(
        input: &str,
        env: &mut Rc<RefCell<Env>>,
    ) -> Result<Exp, SchemeError> {
        let mut buffer = Buffer::from(input);
        let exp = parse_all(&mut buffer)?;
        eval_all(&exp, env)
    }

    #[test]
    fn test_define() {
        let input = "(define a 1) (define b 2) (define c (+ a b)) c";
        let result = evaluate_input(&input).unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(3.0))));
    }

    #[test]
    fn test_let() {
        let result = evaluate_input("(let ((a 1)) a)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));

        let result = evaluate_input("(let ((a 1) (b 2) (c (+ a b))) c)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(3.0))));
    }

    #[test]
    fn test_lambda_fn() {
        let result = evaluate_input("((lambda () 1))").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));

        let result = evaluate_input("((lambda (a b) (+ a b)) 1 2)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(3.0))));
    }

    #[test]
    fn test_define_fn() {
        let result = evaluate_input("(define a (lambda () 1)) (a)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));

        let result = evaluate_input("(define (a) 1) (a)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
    }

    #[test]
    fn test_closures() {
        let mut env = Rc::new(RefCell::new(Env::new()));
        evaluate_input_with_env("(define a 1)", &mut env).unwrap();

        let result = evaluate_input_with_env("((lambda (a) a) 2)", &mut env).unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(2.0))));
        let result = evaluate_input_with_env("a", &mut env).unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));

        let result = evaluate_input_with_env("(let ((a 2)) a)", &mut env).unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(2.0))));
        let result = evaluate_input_with_env("a", &mut env).unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
    }

    #[test]
    fn test_comparisons() {
        let result = evaluate_input("(= 1 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(true)));
        let result = evaluate_input("(= 1 0)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));

        let result = evaluate_input("(> 1 2)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));
        let result = evaluate_input("(> 1 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));

        let result = evaluate_input("(>= 1 2)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));
        let result = evaluate_input("(>= 1 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(true)));

        let result = evaluate_input("(<= 1 2)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(true)));
        let result = evaluate_input("(<= 1 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(true)));
    }

    #[test]
    fn test_if() {
        let result = evaluate_input("(if true 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(0.0))));
        let result = evaluate_input("(if false 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));

        let result = evaluate_input("(if (= 1 1) 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(0.0))));
        let result = evaluate_input("(if (= 1 0) 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
    }

    #[test]
    fn test_and() {
        let result = evaluate_input("(and 1 2 false true)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));
        let result = evaluate_input("(and true 1 2 3)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(3.0))));
    }

    #[test]
    fn test_or() {
        let result = evaluate_input("(or 1 2 false true)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
        let result = evaluate_input("(and false false false false)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(false)));
    }

    #[test]
    fn test_eval_apply() {
        let result = evaluate_input("(eval '(+ 1 1))").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(2.0))));
        let result = evaluate_input("(apply + '(1 1))").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(2.0))));
    }

    #[test]
    fn test_list_construction() {
        let result = evaluate_input("(list)").unwrap();
        assert_eq!(result, Exp::new_list());
        let result = evaluate_input("(list 1 2 3)").unwrap();
        assert_eq!(
            result,
            Exp::List(vec![
                Exp::Atom(Value::Number(Rational::from(1.0))),
                Exp::Atom(Value::Number(Rational::from(2.0))),
                Exp::Atom(Value::Number(Rational::from(3.0)))
            ])
        );

        let result = evaluate_input("(cons 1 '(2 3))").unwrap();
        assert_eq!(
            result,
            Exp::List(vec![
                Exp::Atom(Value::Number(Rational::from(1.0))),
                Exp::Atom(Value::Number(Rational::from(2.0))),
                Exp::Atom(Value::Number(Rational::from(3.0)))
            ])
        );
    }

    #[test]
    fn test_list_access() {
        let result = evaluate_input("(car '(1 2 3))").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
        let result = evaluate_input("(car '())").unwrap_err();
        assert_eq!(
            result,
            SchemeError {
                message: "car called on empty list".to_string()
            }
        );

        let result = evaluate_input("(cdr '(1 2 3))").unwrap();
        assert_eq!(
            result,
            Exp::List(vec![
                Exp::Atom(Value::Number(Rational::from(2.0))),
                Exp::Atom(Value::Number(Rational::from(3.0)))
            ])
        );
        let result = evaluate_input("(cdr '())").unwrap_err();
        assert_eq!(
            result,
            SchemeError {
                message: "cdr called on empty list".to_string()
            }
        );
    }
}
