use garde::Validate;

#[derive(garde::Validate)]
struct Form {
    #[garde(custom(at_least_one))]
    values: Vec<String>,
}

fn at_least_one<T: garde::rules::length::simple::Simple>(value: &T, _: &()) -> garde::Result {
    garde::rules::length::simple::apply(value, (1, usize::MAX))
        .map_err(|_| garde::Error::new("At least one value is required"))
}

#[test]
fn an_empty_field_has_one_error_with_the_requested_message() {
    let report = Form { values: vec![] }.validate().unwrap_err();
    let errors = report.iter().collect::<Vec<_>>();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].0.to_string(), "values");
    assert_eq!(errors[0].1.message(), "At least one value is required");
}

#[test]
fn one_or_more_values_pass() {
    for values in [vec!["one".to_owned()], vec!["one".to_owned(), "two".to_owned()]] {
        assert!(Form { values }.validate().is_ok());
    }
}

#[test]
fn the_wrapped_rule_keeps_the_builtin_length_semantics() {
    for count in 0..=5 {
        let values = vec!["x".to_owned(); count];
        let builtin = garde::rules::length::simple::apply(&values, (1, usize::MAX));
        assert_eq!(at_least_one(&values, &()).is_ok(), builtin.is_ok());
        if let Err(error) = at_least_one(&values, &()) {
            assert_eq!(error.message(), "At least one value is required");
        }
    }
}

#[derive(garde::Validate)]
struct Text {
    #[garde(custom(at_least_one))]
    value: String,
}

#[test]
fn the_same_wrapper_handles_strings() {
    assert_eq!(Text { value: String::new() }.validate().unwrap_err().iter().next().unwrap().1.message(), "At least one value is required");
    assert!(Text { value: "é".to_owned() }.validate().is_ok());
}

#[test]
fn the_builtin_rule_itself_has_not_been_modified() {
    assert_eq!(garde::rules::length::simple::apply(&Vec::<String>::new(), (1, usize::MAX)).unwrap_err().message(), "length is lower than 1");
}
