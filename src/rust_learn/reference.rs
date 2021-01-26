struct T<'a> {
    v: &'a i32,
}

#[test]
fn test() {
    let ref a = 2;
    let b = T { v: a };
    let c = &3;
    let d = T { v: c };
    let ref e: &i32 = &4;
    let f = T { v: e };
    println!("{}", f.v);

    let a = &2;
    let &b = a;
    let c = *a;
}