// #![allow(unused_variables)]
#[derive(Debug)]
struct Cubesat
{
	id: u64,
}

#[derive(Debug)]
enum StatusMessage
{
	Ok,
}

fn	check_status(sat_id: Cubesat) -> StatusMessage
{
	StatusMessage::Ok
}

fn main()
{
	let sat_a = Cubesat{id : 0};
	let sat_b = Cubesat{id : 1};
	let sat_c = Cubesat{id : 2};

	let a_status = check_status(sat_a);
	let b_status = check_status(sat_b);
	let c_status = check_status(sat_c);
	println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

	// "waiting..."
	let a_status = check_status(sat_a);
	let b_status = check_status(sat_b);
	let c_status = check_status(sat_c);
	println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
