use super::*;
use crate::exec::Executor;
use crate::realm::Realm;
use crate::{builtins::value::same_value, forward, forward_val};

#[test]
fn check_boolean_constructor_is_function() {
    let global = Value::new_object(None);
    let boolean_constructor = create(&global);
    assert_eq!(boolean_constructor.is_function(), true);
}

/// Test the correct type is returned from call and construct
#[allow(clippy::result_unwrap_used)]
#[test]
fn construct_and_call() {
    let realm = Realm::create();
    let mut engine = Executor::new(realm);
    let init = r#"
        var one = new Boolean(1);
        var zero = Boolean(0);
        "#;
    eprintln!("{}", forward(&mut engine, init));
    let one = forward_val(&mut engine, "one").unwrap();
    let zero = forward_val(&mut engine, "zero").unwrap();

    assert_eq!(one.is_object(), true);
    assert_eq!(zero.is_boolean(), true);
}

#[test]
fn constructor_gives_true_instance() {
    let realm = Realm::create();
    let mut engine = Executor::new(realm);
    let init = r#"
        var trueVal = new Boolean(true);
        var trueNum = new Boolean(1);
        var trueString = new Boolean("true");
        var trueBool = new Boolean(trueVal);
        "#;

    eprintln!("{}", forward(&mut engine, init));
    let true_val = forward_val(&mut engine, "trueVal").expect("value expected");
    let true_num = forward_val(&mut engine, "trueNum").expect("value expected");
    let true_string = forward_val(&mut engine, "trueString").expect("value expected");
    let true_bool = forward_val(&mut engine, "trueBool").expect("value expected");

    // Values should all be objects
    assert_eq!(true_val.is_object(), true);
    assert_eq!(true_num.is_object(), true);
    assert_eq!(true_string.is_object(), true);
    assert_eq!(true_bool.is_object(), true);

    // Values should all be truthy
    assert_eq!(true_val.is_true(), true);
    assert_eq!(true_num.is_true(), true);
    assert_eq!(true_string.is_true(), true);
    assert_eq!(true_bool.is_true(), true);
}

#[test]
fn instances_have_correct_proto_set() {
    let realm = Realm::create();
    let mut engine = Executor::new(realm);
    let init = r#"
        var boolInstance = new Boolean(true);
        var boolProto = Boolean.prototype;
        "#;

    eprintln!("{}", forward(&mut engine, init));
    let bool_instance = forward_val(&mut engine, "boolInstance").expect("value expected");
    let bool_prototype = forward_val(&mut engine, "boolProto").expect("value expected");

    assert!(same_value(
        &bool_instance.get_internal_slot("__proto__"),
        &bool_prototype,
        true
    ));
}
