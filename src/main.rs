const SCHEME_ENTRY: &str = "_scheme_entry";

fn main() {
    compile_program(42);
}

fn emit_fn(name: String) {
    println!("  .section	__TEXT,__text,regular,pure_instructions");
    println!("  .globl {}", name);
    println!("{}:", name);
}

fn compile_program(val: i64) {
    emit_fn(String::from(SCHEME_ENTRY));
    println!("  movq ${}, %rax", val);
    println!("  retq");
}
