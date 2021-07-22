include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub struct Module {
    module: usize,
}

impl Module {
    unsafe fn new() -> Module {
        Module {
            module: BinaryenModuleCreate(),
        }
    }
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            BinaryenModuleDispose(self.module);
        }
    }
}
