// use clippy linting during unit tests
#![cfg_attr(test, feature(plugin))]
#![cfg_attr(test, plugin(clippy))]
// needed for custom runtime panic handler. rust nightly feature!
#![feature(lang_items)]
// do not link the rust standard library, because that depends on a kernel, which does not yet exist ;).
#![no_std]
// create main entry point during tests on the host system.
#![cfg_attr(not(test), no_main)]

// allow the rust std lib on the host for unit tests for convenience.
#[cfg(test)]
extern crate std;

//allows us to mark special memory regions as having a side-effect.
//this means the compiler will be conservative in optimizing away those
//reads and writes.
extern crate volatile;

//a workaround limitations in the const evaluator of rust. it cannot deal with mutable pointers. so we use a macro instead, that implements the lazy init pattern.
#[macro_use]
extern crate lazy_static;

//because we have not implemented a mutex yet, but need it, we use a cheap stop gap measure by using the simplest, least efficient mutex implementation possible.
//a spin lock. it busy loops around until it gets access to a ressource. this is an efficient way to produce heat.
extern crate spin;

//pulls in print macros, that print to the vga buffer.
#[macro_use]
mod vga_buffer;

// define a custom panic handler for the minimal rust runtime
#[lang = "panic_fmt"]
// do not rename the object symbol of this function, so that we can reference it by name from assembly
#[no_mangle]
// ignore this during unit tests. those typically run on the host systemm where we use the std lib
#[cfg(not(test))]
pub extern "C" fn rust_begin_panic(
  _msg: core::fmt::Arguments,
  _file: &'static str,
  _line: u32,
  _colum: u32,
) -> ! {
  loop {}
}

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn _start() -> ! {
  // this is the entry point into this kernel.
  // since we disabled main, we provide a function, that must be called _start for the linker
  // to find it.
  let nine_dash = "---------";

  println!(
    "{}1{}2{}3{}4{}5{}6{}7{}8",
    nine_dash, nine_dash, nine_dash, nine_dash, nine_dash, nine_dash, nine_dash, nine_dash
  );
  println!(
    "I have processed ...\n...{:4}%\n...{:4}%\n...{:4}%\n...{:4 }%\n... of something!",
    0.0, 40.0, 90.0, 100.0
  );
  println!("N{}seIO{}", 0, 6);
  println!("Adjusting polarities...");
  println!("Switching system duality...");
  println!("ON!");
  println!("Wie wäre es mit ein bißchen deutsch !?");
  loop {}
}
