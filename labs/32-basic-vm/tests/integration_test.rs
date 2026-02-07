// Lab 32: Basic VM - Integration Tests
//
// Tests for the stack-based virtual machine implementation.
// Covers arithmetic, stack operations, comparisons, jumps, function calls,
// error handling, and complete programs (factorial).

use basic_vm::{Instruction, VirtualMachine, VmError};

// ============================================================================
// ARITHMETIC OPERATIONS
// ============================================================================

#[test]
fn test_addition() {
    // 5 + 3 = 8
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[8]);
}

#[test]
fn test_subtraction() {
    // 10 - 3 = 7
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(3),
        Instruction::Sub,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[7]);
}

#[test]
fn test_multiplication() {
    // 4 * 5 = 20
    let program = vec![
        Instruction::Push(4),
        Instruction::Push(5),
        Instruction::Mul,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[20]);
}

#[test]
fn test_division() {
    // 10 / 2 = 5
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(2),
        Instruction::Div,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[5]);
}

#[test]
fn test_compound_arithmetic() {
    // (5 + 3) * 2 = 16
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Add,
        Instruction::Push(2),
        Instruction::Mul,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[16]);
}

#[test]
fn test_negative_result() {
    // 3 - 10 = -7
    let program = vec![
        Instruction::Push(3),
        Instruction::Push(10),
        Instruction::Sub,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[-7]);
}

#[test]
fn test_integer_division_truncates() {
    // 7 / 2 = 3 (integer division)
    let program = vec![
        Instruction::Push(7),
        Instruction::Push(2),
        Instruction::Div,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[3]);
}

// ============================================================================
// STACK OPERATIONS
// ============================================================================

#[test]
fn test_push_and_print() {
    let program = vec![
        Instruction::Push(42),
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[42]);
}

#[test]
fn test_pop() {
    // Push two values, pop one, print remaining
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Pop,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[10]);
}

#[test]
fn test_dup() {
    // Push 5, duplicate, print both
    let program = vec![
        Instruction::Push(5),
        Instruction::Dup,
        Instruction::Print,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[5, 5]);
}

#[test]
fn test_swap() {
    // Push 1 then 2, swap, print both (should print 1 then 2 since swap reverses top two)
    let program = vec![
        Instruction::Push(1),
        Instruction::Push(2),
        Instruction::Swap,
        Instruction::Print, // prints top: 1 (was bottom, now on top after swap)
        Instruction::Print, // prints next: 2
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1, 2]);
}

#[test]
fn test_over() {
    // Push 10, 20, over copies 10 to top: stack becomes [10, 20, 10]
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Over,
        Instruction::Print, // 10 (copy of second)
        Instruction::Print, // 20
        Instruction::Print, // 10 (original)
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[10, 20, 10]);
}

#[test]
fn test_multiple_prints() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Print,
        Instruction::Push(2),
        Instruction::Print,
        Instruction::Push(3),
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1, 2, 3]);
}

// ============================================================================
// COMPARISON OPERATIONS
// ============================================================================

#[test]
fn test_eq_true() {
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(5),
        Instruction::Eq,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1]); // true
}

#[test]
fn test_eq_false() {
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Eq,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[0]); // false
}

#[test]
fn test_lt_true() {
    let program = vec![
        Instruction::Push(3),
        Instruction::Push(5),
        Instruction::Lt,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1]); // 3 < 5 is true
}

#[test]
fn test_lt_false() {
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Lt,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[0]); // 5 < 3 is false
}

#[test]
fn test_gt_true() {
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Gt,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1]); // 5 > 3 is true
}

#[test]
fn test_gt_false() {
    let program = vec![
        Instruction::Push(3),
        Instruction::Push(5),
        Instruction::Gt,
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[0]); // 3 > 5 is false
}

// ============================================================================
// JUMP / CONTROL FLOW
// ============================================================================

#[test]
fn test_unconditional_jump() {
    // Jump over the first Print to only execute the second
    let program = vec![
        Instruction::Push(99),
        Instruction::Jmp(3),       // jump to index 3, skipping Print at index 2
        Instruction::Print,        // skipped
        Instruction::Push(42),
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    // 99 is still on stack but never printed; 42 is pushed and printed
    assert_eq!(vm.output(), &[42]);
}

#[test]
fn test_conditional_jump_taken() {
    // JmpIf with non-zero condition: jump is taken
    let program = vec![
        Instruction::Push(1),      // non-zero = true
        Instruction::JmpIf(3),     // jump to index 3
        Instruction::Halt,         // skipped
        Instruction::Push(100),
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[100]);
}

#[test]
fn test_conditional_jump_not_taken() {
    // JmpIf with zero condition: jump is NOT taken
    let program = vec![
        Instruction::Push(0),      // zero = false
        Instruction::JmpIf(4),     // not taken, fall through
        Instruction::Push(200),
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[200]);
}

#[test]
fn test_if_else_true_branch() {
    // if 5 > 3 then print 100 else print 200
    let program = vec![
        Instruction::Push(5),
        Instruction::Push(3),
        Instruction::Gt,           // 5 > 3? pushes 1
        Instruction::JmpIf(7),     // if true, jump to THEN
        // ELSE (ip = 4-6):
        Instruction::Push(200),
        Instruction::Print,
        Instruction::Jmp(9),       // jump past THEN
        // THEN (ip = 7-8):
        Instruction::Push(100),
        Instruction::Print,
        // END (ip = 9):
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[100]);
}

#[test]
fn test_if_else_false_branch() {
    // if 3 > 5 then print 100 else print 200
    let program = vec![
        Instruction::Push(3),
        Instruction::Push(5),
        Instruction::Gt,           // 3 > 5? pushes 0
        Instruction::JmpIf(7),     // not taken
        // ELSE (ip = 4-6):
        Instruction::Push(200),
        Instruction::Print,
        Instruction::Jmp(9),
        // THEN (ip = 7-8):
        Instruction::Push(100),
        Instruction::Print,
        // END (ip = 9):
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[200]);
}

// ============================================================================
// FUNCTION CALLS (Call / Ret)
// ============================================================================

#[test]
fn test_call_and_ret() {
    // main: push 10, push 20, call add_func, print, halt
    // add_func: add, ret
    let program = vec![
        // MAIN (ip = 0-4):
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Call(5),      // call add_func at ip=5
        Instruction::Print,        // print result (30)
        Instruction::Halt,
        // ADD_FUNC (ip = 5-6):
        Instruction::Add,
        Instruction::Ret,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[30]);
}

#[test]
fn test_nested_function_calls() {
    // main calls func_a, func_a calls func_b
    // func_b pushes 99, returns to func_a, which returns to main
    let program = vec![
        // MAIN (ip = 0-3):
        Instruction::Call(4),      // call func_a
        Instruction::Print,        // print result
        Instruction::Halt,
        Instruction::Halt,         // padding
        // FUNC_A (ip = 4-6):
        Instruction::Call(7),      // call func_b
        Instruction::Ret,
        Instruction::Halt,         // padding
        // FUNC_B (ip = 7-9):
        Instruction::Push(99),
        Instruction::Ret,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[99]);
}

#[test]
fn test_function_with_arguments() {
    // Compute (3 + 7) * 2 using a multiply function
    // main: push 3, push 7, add, push 2, call mul_func, print, halt
    // mul_func: mul, ret
    let program = vec![
        Instruction::Push(3),
        Instruction::Push(7),
        Instruction::Add,          // 10
        Instruction::Push(2),
        Instruction::Call(7),      // call mul_func
        Instruction::Print,        // print 20
        Instruction::Halt,
        // MUL_FUNC (ip = 7):
        Instruction::Mul,
        Instruction::Ret,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[20]);
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[test]
fn test_division_by_zero() {
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(0),
        Instruction::Div,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::DivisionByZero));
}

#[test]
fn test_stack_underflow_pop() {
    let program = vec![
        Instruction::Pop,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_stack_underflow_add() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Add, // only one value on stack
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_stack_underflow_dup_empty() {
    let program = vec![
        Instruction::Dup,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_stack_underflow_swap() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Swap, // only one value
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_stack_underflow_over() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Over, // only one value
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_stack_underflow_print() {
    let program = vec![
        Instruction::Print,
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::StackUnderflow));
}

#[test]
fn test_invalid_jump() {
    let program = vec![
        Instruction::Jmp(100), // out of bounds
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::InvalidJump(100)));
}

#[test]
fn test_invalid_call_target() {
    let program = vec![
        Instruction::Call(50), // out of bounds
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::InvalidJump(50)));
}

#[test]
fn test_call_stack_underflow() {
    let program = vec![
        Instruction::Ret, // no call on stack
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::CallStackUnderflow));
}

// ============================================================================
// HALT
// ============================================================================

#[test]
fn test_halt_stops_execution() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Print,
        Instruction::Halt,
        Instruction::Push(2),  // should never execute
        Instruction::Print,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1]);
    assert!(vm.is_halted());
}

#[test]
fn test_empty_program() {
    // IP starts at 0, but code is empty, so step should fail
    let program = vec![];
    let mut vm = VirtualMachine::new(program);
    let result = vm.run();
    assert_eq!(result, Err(VmError::InvalidJump(0)));
}

// ============================================================================
// COMPLEX PROGRAMS
// ============================================================================

#[test]
fn test_factorial_5() {
    // factorial(5) = 120 using a loop
    // Loop invariant at ip=2: stack = [..., n, acc] (acc on top)
    //
    // Trace through iterations:
    //   [5, 1] -> [4, 5] -> [3, 20] -> [2, 60] -> [1, 120] -> print 120
    let program = vec![
        Instruction::Push(5),        // 0: n = 5
        Instruction::Push(1),        // 1: acc = 1. Stack: [5, 1]
        // LOOP (ip = 2): stack = [..., n, acc] (acc on top)
        Instruction::Over,           // 2: [..., n, acc, n]
        Instruction::Push(1),        // 3: [..., n, acc, n, 1]
        Instruction::Eq,             // 4: [..., n, acc, n==1]
        Instruction::JmpIf(13),      // 5: if n==1, jump to END. Stack: [..., n, acc]
        // BODY: stack = [..., n, acc]
        Instruction::Over,           // 6: [..., n, acc, n]
        Instruction::Mul,            // 7: [..., n, acc*n]
        Instruction::Swap,           // 8: [..., acc*n, n]
        Instruction::Push(1),        // 9: [..., acc*n, n, 1]
        Instruction::Sub,            // 10: [..., acc*n, n-1]
        Instruction::Swap,           // 11: [..., n-1, acc*n] = [new_n, new_acc]
        Instruction::Jmp(2),         // 12: back to LOOP
        // END (ip = 13): stack = [..., n, acc] where n==1
        Instruction::Swap,           // 13: [..., acc, n]
        Instruction::Pop,            // 14: [..., acc]
        Instruction::Print,          // 15: print acc
        Instruction::Halt,           // 16
    ];

    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[120]);
}

#[test]
fn test_factorial_1() {
    // factorial(1) = 1 (edge case: loop body never executes)
    let program = vec![
        Instruction::Push(1),        // 0: n = 1
        Instruction::Push(1),        // 1: acc = 1. Stack: [1, 1]
        Instruction::Over,           // 2: [1, 1, 1]
        Instruction::Push(1),        // 3: [1, 1, 1, 1]
        Instruction::Eq,             // 4: [1, 1, 1]
        Instruction::JmpIf(13),      // 5: taken -> [1, 1]
        Instruction::Over,           // 6: (not reached)
        Instruction::Mul,            // 7
        Instruction::Swap,           // 8
        Instruction::Push(1),        // 9
        Instruction::Sub,            // 10
        Instruction::Swap,           // 11
        Instruction::Jmp(2),         // 12
        Instruction::Swap,           // 13: [1, 1]
        Instruction::Pop,            // 14: [1]
        Instruction::Print,          // 15: output 1
        Instruction::Halt,           // 16
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1]);
}

// ============================================================================
// ACCESSOR METHODS
// ============================================================================

#[test]
fn test_stack_accessor() {
    let program = vec![
        Instruction::Push(10),
        Instruction::Push(20),
        Instruction::Push(30),
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.stack(), &[10, 20, 30]);
}

#[test]
fn test_ip_accessor() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Push(2),
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    assert_eq!(vm.ip(), 0);
    vm.step().unwrap();
    assert_eq!(vm.ip(), 1);
    vm.step().unwrap();
    assert_eq!(vm.ip(), 2);
}

#[test]
fn test_is_halted() {
    let program = vec![
        Instruction::Push(1),
        Instruction::Halt,
    ];
    let mut vm = VirtualMachine::new(program);
    assert!(!vm.is_halted());
    vm.step().unwrap(); // Push
    assert!(!vm.is_halted());
    vm.step().unwrap(); // Halt
    assert!(vm.is_halted());
}

// ============================================================================
// LOOP COUNTING
// ============================================================================

#[test]
fn test_simple_loop_counts_to_3() {
    // Push values 1, 2, 3 and print each using a loop
    // counter starts at 1, loop while counter <= 3
    let program = vec![
        Instruction::Push(1),        // 0: counter = 1
        // LOOP (ip = 1):
        Instruction::Dup,            // 1: [counter, counter]
        Instruction::Print,          // 2: print counter. Stack: [counter]
        Instruction::Push(1),        // 3: [counter, 1]
        Instruction::Add,            // 4: [counter+1]
        Instruction::Dup,            // 5: [counter+1, counter+1]
        Instruction::Push(4),        // 6: [counter+1, counter+1, 4]
        Instruction::Lt,             // 7: counter+1 < 4? Stack: [counter+1, result]
        Instruction::JmpIf(1),       // 8: if true, jump to LOOP. Stack: [counter+1]
        Instruction::Pop,            // 9: clean up
        Instruction::Halt,           // 10
    ];
    let mut vm = VirtualMachine::new(program);
    vm.run().unwrap();
    assert_eq!(vm.output(), &[1, 2, 3]);
}
