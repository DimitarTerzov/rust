fn main () {
	let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
	let taxonomy = ["Animalia", "Arthropodia", "Insecta"];
	
	assert_eq! (lazy_caterer[3], 7);
	assert_eq! (taxonomy.len(), 3);
	
	let mut sieve = [true; 100000];
	for i in 2..100 {
		if sieve[i] {
			let mut j = i * i;
			while j < 100000{
				sieve[j] = false;
				j += i;
			}
		}
	}
	
	assert!(sieve[211]);
	assert!(!sieve[30031]);
	
	let mut chaos = [3, 5, 4, 1, 2];
	chaos.sort();
	assert_eq!(chaos, [1, 2, 3, 4, 5]);
	
	let mut v = vec![2, 3, 5, 7];
	assert_eq!(v.iter().fold(1, |a, b| a * b), 210);
	v.push(11);
	v.push(13);
	assert_eq!(v.iter().fold(1, |a, b| a * b), 30030);
	
	let mut c = Vec::new();
	c.push("step");
	c.push("on");
	c.push("no");
	c.push("pets");
	assert_eq!(c, vec!["step", "on", "no", "pets"]);
	
	let d: Vec<i32> = (0..5).collect();
	assert_eq!(d, [0, 1, 2, 3, 4]);
	
	let mut vec = Vec::new();
	vec.push(7);
	vec.extend([1, 2, 3].iter().cloned());
	
	for x in &vec {
		println!("{}", x);
	}
	assert_eq!(vec, [7, 1, 2, 3]);
	
	let mut vec = vec![1, 2, 3];
	vec.push(4);
	assert_eq!(vec, [1, 2, 3, 4]);
	
	let vec = vec![0; 5];
	assert_eq!(vec, [0, 0, 0, 0, 0]);
	
	let mut stack = Vec::new();
	stack.push(1);
	stack.push(2);
	stack.push(3);
	
	while let Some(top) = stack.pop() {
		println!("{}", top);
	}
}