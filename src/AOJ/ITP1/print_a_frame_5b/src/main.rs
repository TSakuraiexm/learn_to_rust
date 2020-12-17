struct Rectangle {
	h: u32,
	w: u32,
}

impl Rectangle {
	fn draw(&self) {
		for _i in 0..self.h {
			if _i == 0 || _i == (self.h - 1){
				for _n in 0..self.w {
					print!("#");
				}
			} else {
				for _n in 0..self.w {
					if _n == 0 || _n == (self.w - 1) {
						print!("#");
					} else {
						print!(".")
					}
				}
			}
			println!("");
		}
		println!("");
	}
}

fn main() {
	let mut rect_vec: Vec<Rectangle> = Vec::new();

	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let mut input = input.split_whitespace();

		let temp = Rectangle {
			h: input.next().unwrap().parse().unwrap(),
			w: input.next().unwrap().parse().unwrap(),
		};

		if temp.h == 0 && temp.w == 0 {
			break;
		}

		rect_vec.push(temp);
	}

	for i in rect_vec {
		i.draw();
	}
}
