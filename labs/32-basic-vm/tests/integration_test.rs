//! Integration tests for Lab 32: Basic VM
//!
//! These tests verify the `VM`'s execution of various programs,
//! checking for correctness of arithmetic, stack manipulation, control flow,
//! and error handling.

use basic_vm::solution::{Instruction, VM, VmError};

/// Helper to run a program and assert that it returns a specific value.
fn assert_program_result(program: Vec<Instruction>, expected: i32) {
    let mut vm = VM::new(program);
    assert_eq!(vm.run().unwrap(), Some(expected));
}

/// Helper to run a program and assert that it returns a specific error.
fn assert_program_error(program: Vec<Instruction>, expected_error: VmError) {
    let mut vm = VM::new(program);
    assert_eq!(vm.run().unwrap_err(), expected_error);
}

// ============================================================================
// ARITHMETIC TESTS
// ============================================================================

#[test]
fn test_addition() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Add, Instruction::Halt];
    assert_program_result(program, 15);
}

#[test]
fn test_subtraction() {
    let program = vec![Instruction::Push(10), Instruction::Push(5), Instruction::Sub, Instruction::Halt];
    assert_program_result(program, 5);
}

#[test]
fn test_multiplication() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Mul, Instruction::Halt];
    assert_program_result(program, 50);
}

#[test]
fn test_division() {
    let program = vec![Instruction::Push(10), Instruction::Push(5), Instruction::Div, Instruction::Halt];
    assert_program_result(program, 2);
}

#[test]
fn test_integer_division_truncates() {
    let program = vec![Instruction::Push(10), Instruction::Push(3), Instruction::Div, Instruction::Halt];
    assert_program_result(program, 3);
}

#[test]
fn test_compound_arithmetic() {
    // (5 + 10) * 2 / 3
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(10),
        Instruction::Add,
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Push(3),
        Instruction::Div,
        Instruction::Halt,
    ];
    assert_program_result(program, 10);
}

// ============================================================================
// STACK MANIPULATION TESTS
// ============================================================================

#[test]
fn test_pop() {
    let program = vec![Instruction::Push(1), Instruction::Push(2), Instruction::Pop, Instruction::Halt];
    assert_program_result(program, 1);
}

#[test]
fn test_dup() {
    let program = vec![Instruction::Push(5), Instruction::Dup, Instruction::Add, Instruction::Halt];
    assert_program_result(program, 10);
}

#[test]
fn test_swap() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Swap, Instruction::Sub, Instruction::Halt];
    assert_program_result(program, 5); // 10 - 5
}

#[test]
fn test_over() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Over, Instruction::Add, Instruction::Halt];
    assert_program_result(program, 15); // 10 + 5
}

// ============================================================================
// COMPARISON TESTS
// ============================================================================

#[test]
fn test_eq_true() {
    let program = vec![Instruction::Push(5), Instruction::Push(5), Instruction::Eq, Instruction::Halt];
    assert_program_result(program, 1);
}

#[test]
fn test_eq_false() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Eq, Instruction::Halt];
    assert_program_result(program, 0);
}

#[test]
fn test_gt_true() {
    let program = vec![Instruction::Push(10), Instruction::Push(5), Instruction::Gt, Instruction::Halt];
    assert_program_result(program, 1);
}

#[test]
fn test_gt_false() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Gt, Instruction::Halt];
    assert_program_result(program, 0);
}

#[test]
fn test_lt_true() {
    let program = vec![Instruction::Push(5), Instruction::Push(10), Instruction::Lt, Instruction::Halt];
    assert_program_result(program, 1);
}

#[test]
fn test_lt_false() {
    let program = vec![Instruction::Push(10), Instruction::Push(5), Instruction::Lt, Instruction::Halt];
    assert_program_result(program, 0);
}

// ============================================================================
// CONTROL FLOW TESTS
// ============================================================================

#[test]
fn test_unconditional_jump() {
    let program = vec![Instruction::Push(1), Instruction::Jmp(3), Instruction::Push(100), Instruction::Push(99), Instruction::Halt];
    assert_program_result(program, 99);
}

#[test]
fn test_conditional_jump_taken() {
    let program = vec![Instruction::Push(1), Instruction::JmpIf(3), Instruction::Push(100), Instruction::Push(99), Instruction::Halt];
    assert_program_result(program, 99);
}

#[test]
fn test_conditional_jump_not_taken() {
    let program = vec![Instruction::Push(0), Instruction::JmpIf(3), Instruction::Push(100), Instruction::Halt];
    assert_program_result(program, 100);
}

#[test]
fn test_simple_loop() {
    // n = 3; while n > 0 { n = n-1 }; result is 0
    let program_simple_loop = vec![
        Instruction::Push(3), // n
        // loop start (ip=1)
        Instruction::Dup, // [n, n]
        Instruction::Push(0),
        Instruction::Gt, // [n, n>0]
        Instruction::JmpIf(6), // if n > 0, jump to loop body
        Instruction::Halt, // else halt
        // loop body (ip=6)
        Instruction::Push(1),
        Instruction::Sub, // n = n-1
        Instruction::Jmp(1), // jmp to loop start
    ];
    assert_program_result(program_simple_loop, 0);
}


// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_stack_underflow_add() {
    let program = vec![Instruction::Push(5), Instruction::Add, Instruction::Halt];
    assert_program_error(program, VmError::StackUnderflow);
}

#[test]
fn test_stack_underflow_pop() {
    let program = vec![Instruction::Pop, Instruction::Halt];
    assert_program_error(program, VmError::StackUnderflow);
}

#[test]
fn test_division_by_zero() {
    let program = vec![Instruction::Push(10), Instruction::Push(0), Instruction::Div, Instruction::Halt];
    assert_program_error(program, VmError::DivisionByZero);
}

#[test]
fn test_invalid_jump_pointer() {
    let program = vec![Instruction::Jmp(100)];
    let mut vm = VM::new(program);
    assert_eq!(vm.run().unwrap_err(), VmError::InvalidInstructionPointer);
}

#[test]
fn test_program_ends_without_halt() {
    let program = vec![Instruction::Push(42)];
    let mut vm = VM::new(program);
    assert_eq!(vm.run().unwrap(), Some(42));
}

#[test]
fn test_empty_program() {
    let program = vec![];
    let mut vm = VM::new(program);
    assert_eq!(vm.run().unwrap(), None);
}
