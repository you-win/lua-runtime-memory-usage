#[global_allocator]
static PEAK_ALLOC: peak_alloc::PeakAlloc = peak_alloc::PeakAlloc;

const CODE: &str = r#"
    local count = 0
    for i = 1, 1000 do
        count = count + 1
    end

    return count
"#;

fn measure() {
    current_mem_use();

    let lua = mlua::Lua::new();
    let count = lua.load(CODE).set_name("test loop").eval::<i32>().unwrap();

    current_mem_use();

    assert!(count == 1000);
}

fn current_mem_use() {
    let current_mem = PEAK_ALLOC.current_usage_as_kb();

    println!("current mem usage (kb): {current_mem}");
}

fn main() {
    measure();
}
