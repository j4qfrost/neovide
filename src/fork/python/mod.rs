use rustpython_compiler::mode::Mode;
use rustpython_vm::function::PyFuncArgs;
use rustpython_vm::pyobject::PyResult;
use rustpython_vm::{Interpreter, VirtualMachine};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Default)]
pub struct Python {
    runtime: Interpreter,
}

impl Python {
    pub fn init(&mut self) {
        let mut display_root = PathBuf::new();
        display_root.push(env!("CARGO_MANIFEST_DIR"));
        display_root.push("src/fork/python/scripts/hello_world.py");
        let file_path = display_root.to_str().unwrap();
        let file = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let hello = self
            .runtime
            .enter(|vm| vm.compile(&contents, Mode::Exec, file_path.to_string()))
            .unwrap();

        self.runtime
            .enter(|vm| {
                let scope = vm.new_scope_with_builtins();
                vm.run_code_obj(hello, scope)?;
                let function = vm.ctx.new_function(test_func);
                let args = PyFuncArgs::new([(vm.ctx.new_int(5))].to_vec(), Default::default());
                vm.invoke(&function, args)
            })
            .unwrap();
    }
}

fn test_func(mut py_args: PyFuncArgs, vm: &VirtualMachine) -> PyResult {
    let five = vm.ctx.new_int(5);
    let ret = vm._add(&py_args.shift(), &five);
    println!("{:?}", 5);
    ret
}
