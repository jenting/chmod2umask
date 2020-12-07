use std::collections::HashMap;
use std::env;

fn help() {
	println!("chmod2umask is a tool for converts the `chmod` file mode to the `umask` symbolic mode.

Usage:

  chmod2umask <filemode>
");
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		// one argument passed
		2 => {
			match args[1].parse() {
				Ok(filemode) => {
					match filemode_to_symbolicmode(filemode) {
						Ok(v) => println!("{}", v),
						Err(e) => println!("{}", e),
					}
                },
				Err(_) => {
					eprintln!("error: file mode not an integer");
					help();
					return;
				},
			};
		},
		// all the other cases
		_ => {
			// show a help message
			help();
		}
	}
}

// filemode_to_symbolicmode converts file mode to symbolic mode
fn filemode_to_symbolicmode(filemode: u32)  -> Result<String, Box<dyn std::error::Error>> {
	let o_filemode: u32 = filemode % 10;
	let g_filemode: u32 = (filemode / 10) % 10;
	let u_filemode: u32 = (filemode / 100) % 10;

	if o_filemode > 7 || g_filemode > 7 || u_filemode > 7 {
		return Err(format!("invalid file mode: {}", filemode).into())
	}

	let mut map: HashMap<u32, String> = HashMap::new();
	map.insert(0, "".to_string());
	map.insert(1, "x".to_string());
	map.insert(2, "w".to_string());
	map.insert(3, "wx".to_string());
	map.insert(4, "r".to_string());
	map.insert(5, "rx".to_string());
	map.insert(6, "rw".to_string());
	map.insert(7, "rwx".to_string());
	map.insert(8, "".to_string());
	map.insert(9, "".to_string());

	return Ok(format!("u={},g={},o={}", 
		map.get(&u_filemode).unwrap(),
		map.get(&g_filemode).unwrap(),
		map.get(&o_filemode).unwrap()));
}

#[test]
fn test_filemode_0777() {
	assert_eq!(filemode_to_symbolicmode(0777).unwrap(), "u=rwx,g=rwx,o=rwx")
}

#[test]
fn test_filemode_0644() {
	assert_eq!(filemode_to_symbolicmode(644).unwrap(), "u=rw,g=r,o=r")
}

#[test]
fn test_filemode_0000() {
	assert_eq!(filemode_to_symbolicmode(0).unwrap(), "u=,g=,o=")
}

#[test]
#[should_panic]
fn test_filemode_0844() {
    filemode_to_symbolicmode(0844).unwrap();
}
