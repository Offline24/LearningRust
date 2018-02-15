extern crate variables_and_mutability;

fn main() {
    run_with_messages("let_is_immutable", &variables_and_mutability::let_is_immutable);
    run_with_messages("", &variables_and_mutability::let_is_immutable);
    run_with_messages("let_with_mut_is_mutable", &variables_and_mutability::let_with_mut_is_mutable);
    run_with_messages("mutable_arguments", &variables_and_mutability::mutable_arguments);
}

fn run_with_messages(name: &str, function: &Fn() -> ()) {
    println!("--- {}", name);
    function();
    println!("---");
    println!("");
}