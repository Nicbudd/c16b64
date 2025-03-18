use std::io::{self, Write};

fn main() {
	let mut stack: Vec<u16> = Vec::new();
	let mut flag = false;

	// instr 5
	stack.push(0x4fda);

	// instr r
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_right(1));

	// instr 6
	stack.push(0x20fe);

	// instr 1
	stack.push(0x14bc);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr A
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	stack.push(x & y);

	// instr a
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	let sum = x.overflowing_add(y);
	flag = sum.1;
	let sum = sum.0;
	stack.push(sum);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 3
	stack.push(0x7e37);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr 3
	stack.push(0x7e37);

	// instr 3
	stack.push(0x7e37);

	// instr R
	let x = stack.pop().expect("No items on stack to shift.") as u32;
	let y = stack.pop().expect("Not enough items on stack to shift.");
	stack.push(y.rotate_right(x));

	// instr A
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	stack.push(x & y);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 7
	stack.push(0x445a);

	// instr N
	let x = stack.pop().expect("No items on stack to preform NOT operation on.");

stack.push(!x);

	// instr 9
	stack.push(0x25e5);

	// instr 2
	stack.push(0xfc26);

	// instr X
	let x = stack.pop().expect("No items on stack to XOR.");
	let y = stack.pop().expect("Not enough items on stack to XOR.");
	stack.push(x ^ y);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr a
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	let sum = x.overflowing_add(y);
	flag = sum.1;
	let sum = sum.0;
	stack.push(sum);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 0
	stack.push(0x1c72);

	// instr 8
	stack.push(0xb76a);

	// instr 1
	stack.push(0x14bc);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr X
	let x = stack.pop().expect("No items on stack to XOR.");
	let y = stack.pop().expect("Not enough items on stack to XOR.");
	stack.push(x ^ y);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr X
	let x = stack.pop().expect("No items on stack to XOR.");
	let y = stack.pop().expect("Not enough items on stack to XOR.");
	stack.push(x ^ y);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 5
	stack.push(0x4fda);

	// instr 8
	stack.push(0xb76a);

	// instr 5
	stack.push(0x4fda);

	// instr r
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_right(1));

	// instr a
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	let sum = x.overflowing_add(y);
	flag = sum.1;
	let sum = sum.0;
	stack.push(sum);

	// instr N
	let x = stack.pop().expect("No items on stack to preform NOT operation on.");

stack.push(!x);

	// instr X
	let x = stack.pop().expect("No items on stack to XOR.");
	let y = stack.pop().expect("Not enough items on stack to XOR.");
	stack.push(x ^ y);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 0
	stack.push(0x1c72);

	// instr 4
	stack.push(0xb53f);

	// instr 2
	stack.push(0xfc26);

	// instr l
	let x = stack.pop().expect("No items on stack to shift.");
	stack.push(x.rotate_left(1));

	// instr A
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	stack.push(x & y);

	// instr N
	let x = stack.pop().expect("No items on stack to preform NOT operation on.");

stack.push(!x);

	// instr a
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	let sum = x.overflowing_add(y);
	flag = sum.1;
	let sum = sum.0;
	stack.push(sum);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");

	// instr 0
	stack.push(0x1c72);

	// instr 1
	stack.push(0x14bc);

	// instr 5
	stack.push(0x4fda);

	// instr A
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	stack.push(x & y);

	// instr a
	let x = stack.pop().expect("No items on stack to add.");
	let y = stack.pop().expect("Not enough items on stack to add.");
	let sum = x.overflowing_add(y);
	flag = sum.1;
	let sum = sum.0;
	stack.push(sum);

	// instr C
	let top_val = stack.pop().expect("No items on stack to print.");
	let x: char = (top_val & 0xff) as u8 as char;
	let y: char = (top_val >> 8) as u8 as char;
	let mut x_utf8 = [0; 2];
	let mut y_utf8 = [0; 2];
	x.encode_utf8(&mut x_utf8);
	y.encode_utf8(&mut y_utf8);
	io::stdout().write(&y_utf8).expect("IO Error");
	io::stdout().write(&x_utf8).expect("IO Error");
}