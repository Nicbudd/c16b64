#[allow(non_upper_case_globals)]
pub const instr_A: &'static str = "let x = stack.pop().expect(\"No items on stack to add.\");
\tlet y = stack.pop().expect(\"Not enough items on stack to add.\");
\tstack.push(x & y);";

#[allow(non_upper_case_globals)]
pub const instr_C: &'static str = "let top_val = stack.pop().expect(\"No items on stack to print.\");
\tlet x: char = (top_val & 0xff) as u8 as char;
\tlet y: char = (top_val >> 8) as u8 as char;
\tlet mut x_utf8 = [0; 2];
\tlet mut y_utf8 = [0; 2];
\tx.encode_utf8(&mut x_utf8);
\ty.encode_utf8(&mut y_utf8);
\tio::stdout().write(&y_utf8).expect(\"IO Error\");
\tio::stdout().write(&x_utf8).expect(\"IO Error\");";

#[allow(non_upper_case_globals)]
pub const instr_L: &'static str = "let x = stack.pop().expect(\"No items on stack to shift.\") as u32;
\tlet y = stack.pop().expect(\"Not enough items on stack to shift.\");
\tstack.push(y.rotate_left(x));";

#[allow(non_upper_case_globals)]
pub const instr_N: &'static str = "let x = stack.pop().expect(\"No items on stack to preform NOT operation on.\");
\nstack.push(!x);";

#[allow(non_upper_case_globals)]
pub const instr_R: &'static str = "let x = stack.pop().expect(\"No items on stack to shift.\") as u32;
\tlet y = stack.pop().expect(\"Not enough items on stack to shift.\");
\tstack.push(y.rotate_right(x));";

#[allow(non_upper_case_globals)]
pub const instr_X: &'static str = "let x = stack.pop().expect(\"No items on stack to XOR.\");
\tlet y = stack.pop().expect(\"Not enough items on stack to XOR.\");
\tstack.push(x ^ y);";

#[allow(non_upper_case_globals)]
pub const instr_a: &'static str = "let x = stack.pop().expect(\"No items on stack to add.\");
\tlet y = stack.pop().expect(\"Not enough items on stack to add.\");
\tlet sum = x.overflowing_add(y);
\tflag = sum.1;
\tlet sum = sum.0;
\tstack.push(sum);";

#[allow(non_upper_case_globals)]
pub const instr_l: &'static str = "let x = stack.pop().expect(\"No items on stack to shift.\");
\tstack.push(x.rotate_left(1));";

#[allow(non_upper_case_globals)]
pub const instr_r: &'static str = "let x = stack.pop().expect(\"No items on stack to shift.\");
\tstack.push(x.rotate_right(1));";