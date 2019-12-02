// A "VM" for the intcode machine, which sounds like it's going to be a thing
// in at least a few of the problems this year

#[derive(Debug)]
pub struct IntcodeVM {
	memory: Vec<i32>,
	ptr: i32,
}

impl IntcodeVM {
	pub fn dump(self) {
		println!("{}", self.memory.len());
		println!("{:?}", self.memory);
	}

	pub fn read(&self, loc: i32) -> i32 {
		self.memory[loc as usize]
	}

	fn write (&mut self, loc: i32, val: i32) {
		self.memory[loc as usize] = val;
	}

	pub fn run(&mut self, noun: i32, verb: i32) {
		self.write(1, noun);
		self.write(2, verb);
	
		self.ptr = 0;
		loop {
			match self.read(self.ptr) {
				// add and write back
				1  => {
					let a = self.read(self.ptr+1);
					let b = self.read(self.ptr+2);
					let dest = self.read(self.ptr+3);
					self.write(dest, self.read(a) + self.read(b));
				},
				// multiply and write back
				2  => {
					let a = self.read(self.ptr+1);
					let b = self.read(self.ptr+2);
					let dest = self.read(self.ptr+3);
					self.write(dest, self.read(a) * self.read(b));
				},
				// halt!
				99 => break,
				// I don't think this should ever happen with our input?
				_  => panic!("Hmm this shouldn't happen..."),
			}
			self.ptr += 4;
		}
	}

	pub fn new() -> IntcodeVM {
		IntcodeVM { ptr: 0, memory: Vec::new() }
	}

	pub fn load(&mut self, prog_txt: &str) {
		self.memory = prog_txt.split(",")
			.map(|a| a.parse::<i32>().unwrap()).collect();		
	}
}