This is a guide on how to use this repo

1. The completed folder contains past solutions. Whenever a new solution is completed add it to the folder 

2. Problems currently being worked on are directly in the Kata folder

3. To build, run or test a current problem's solution, change the path of the current project inside the cargo toml, under the bin section. 

Use this command and replace the NewProblemName with the target project name: 

sed -i 's/path = "\/home\/ddr\/Repos\/SandBox\/Rust\/Kata\/\(.*\).rs"/path = "\/home\/ddr\/Repos\/SandBox\/Rust\/Kata\/NewProblemName.rs"/g' ./Cargo.toml

4. Once that is setup you can use these commands 

	cargo build
	cargo run
	cargo test
