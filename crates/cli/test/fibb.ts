@external("env", "wasm_input")
declare function wasm_input(x: i32): i64

@external("env", "require")
declare function require(x: i32): void

export function read_public_input(): i64 {
    return wasm_input(1);
}

export function read_private_input(): i64 {
    return wasm_input(0);
}

export function fibb(n: i64): i64 {
    if (n == 0) return 0;
    if (n == 1) return 1;
    let a: i64 = 0;
    let b: i64 = 1;
    for (let i: i64 = 2; i <= n; ++i)
    {
        let tmp: i64 = a + b;
        a = b;
        b = tmp;
    }
    return b;
}

export function zkmain(): void {
    let n = read_private_input();
    let res = fibb(n);
    let expected_res = read_public_input();

    require(res == expected_res);
}
