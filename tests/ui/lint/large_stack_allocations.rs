// check-fail

static _T: [u8; 1 << 24] = [0; 1 << 24];
//~^ WARN large stack allocation
static _T2: [u8; 1 << 24] = { [0; 1 << 24] };
//~^ WARN large stack allocation

fn test() -> [u8; 1 << 32] {
    [0; 1 << 32]
    //~^ ERROR dangerous large stack allocation
}

fn foo<A: Copy>(a: A) -> [A; 1 << 32] {
    [a; 1 << 32]
}

fn main() {
    let _a = [0u32; 1 << 32];
    //~^ ERROR dangerous large stack allocation
    let _b = test()[0];
    //~^ ERROR dangerous large stack allocation
    let _c: [u8; 1 << 24] = std::array::from_fn(|_| 0);
    //~^ WARN large stack allocation

    match _a {
        [a, ..] if u32::from(test()[0]) == a => {},
    //~^ ERROR dangerous large stack allocation
        _ => {}
    }

    let _d = foo(3);
    //~^ ERROR dangerous large stack allocation

    struct A<T>(T);
    let _e = A(_a);
    //~^ ERROR dangerous large stack allocation
    let _f = A([0; 1024 * 1024 * 10]);
    //~^ WARN large stack allocation
    //~^^ WARN large stack allocation
}
