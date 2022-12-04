mod integration_tests {
    use crate::evaluator::eval_all;
    use crate::parser::parse_all;
    use crate::types::rational::Rational;
    use crate::types::Exp;
    use crate::{lib::SchemeError, types::Value};
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{environment::Env, evaluator::evaluate, parser::parse, tokenizer::tokenize};

    fn evaluate_input(input: &str) -> Result<Exp, SchemeError> {
        let mut env = Rc::new(RefCell::new(Env::new()));
        let mut tokens = tokenize(input)?;
        let exp = parse_all(&mut tokens)?;
        eval_all(&exp, &mut env)
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
    fn test_comparisons() {
        let result = evaluate_input("(eq? 1 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Boolean(true)));
        let result = evaluate_input("(eq? 1 0)").unwrap();
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

        let result = evaluate_input("(if (eq? 1 1) 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(0.0))));
        let result = evaluate_input("(if (eq? 1 0) 0 1)").unwrap();
        assert_eq!(result, Exp::Atom(Value::Number(Rational::from(1.0))));
    }

    #[test]
    fn test_and() {
        todo!()
    }

    #[test]
    fn test_or() {
        todo!()
    }
}
