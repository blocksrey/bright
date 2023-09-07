fn constant_speed_interpolation(p: f32, t: f32, s: f32, dt: f32) -> f32 {
	let o = t - p;
	let z = dt*s;


	if z*z < o*o {
		return p + z.copysign(o);
	}

	return t;
}

fn bv_to_lum(bv: f32) -> f32 {
	return bv.powf(1.0/3.0);
}

fn lum_to_bv(lum: f32) -> f32 {
	return lum.powf(3.0);
}

fn main() {
	/*
	let mut dir: String = "".to_string();
	for _dir in std::fs::read_dir("/sys/class/backlight/").unwrap() {
		dir = _dir.unwrap().path().display().to_string();
		break;
	}
	*/

	let backlight_path_string =
	std::fs::read_dir("/sys/class/backlight/")
	.unwrap()
	.next()
	.unwrap()
	.unwrap()
	.path()
	.display()
	.to_string();

	let brightness_path_string = format!("{}/brightness", backlight_path_string);
	let max_brightness_path_string = format!("{}/max_brightness", backlight_path_string);

	let brightness =
	std::fs::read_to_string(&brightness_path_string) // I don't really get why I need the address here
	.unwrap()
	.trim_end()
	.parse::<f32>()
	.unwrap();

	let max_brightness =
	std::fs::read_to_string(max_brightness_path_string) // Do I need to use the address here?
	.unwrap()
	.trim_end()
	.parse::<f32>()
	.unwrap();

	let args: Vec<String> = std::env::args().collect();

	let mut scale_pos = brightness/max_brightness;
	let scale_tar = args[1].parse::<f32>().unwrap().min(1.0).max(0.0);

	let step_tar = (max_brightness*scale_tar).round();
	println!("{} -> {}", scale_pos, scale_tar);
	println!("Round target {}", step_tar);

	let speed = 0.1;

	let dt = 0.01; // Pretty much equivalent to dt because there's like zero overhead in the loop
	let loop_duration = std::time::Duration::from_millis((1000.0*dt) as u64);

	//println!("{}", brightness_path_string);

	let brightness_path_buf = std::path::PathBuf::from(brightness_path_string);

	loop {
		//scale_pos += dt*(scale_tar - scale_pos);
		scale_pos = constant_speed_interpolation(scale_pos, scale_tar, speed, dt);
		println!("{}", scale_pos);

		let step_pos = (max_brightness*scale_pos).round();
		std::fs::write(&brightness_path_buf, step_pos.to_string()).unwrap();

		if step_pos == step_tar {
			break;
		}

		std::thread::sleep(loop_duration);
	}
}