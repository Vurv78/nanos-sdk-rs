use std::path::PathBuf;

#[derive(Debug)]
enum SdkGeneratorError {
	OutDirNotFound,
	BindgenFailure,
	WriteFailure( std::io::Error )
}

fn main() -> Result<(), SdkGeneratorError> {
	use SdkGeneratorError::*;

	let out_dir = PathBuf::from( std::env::var("OUT_DIR").map_err( |_| OutDirNotFound )? );

	let bindings = bindgen::Builder::default()
		.header("module-sdk/include/lua.hpp")
		.allowlist_function("(LUA|lua).*")
		.allowlist_type("(LUA|lua).*")
		.allowlist_var("(LUA|lua).*")
		.generate()
		.map_err(|_| BindgenFailure)?;

	bindings
		.write_to_file(out_dir.join("bindings.rs"))
		.map_err(WriteFailure)?;

	println!( "cargo:rustc-link-search=native=module-sdk/lib" );
	println!( "cargo:rustc-link-lib=static=lua" );

	Ok(())
}