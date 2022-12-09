struct A {
    x: i32,
    y: i32,
}

struct B {
    a: i32,
    b: i32,
    c:i32,
}



fn main() {
    
    let vec_a = vec![
        A {x:1,y:2},
        A {x:1,y:2},
        A {x:1,y:2}
    ];

    for i in 0..vec_a.len(){
        let b = B { a:vec_a[i].x, b:vec_a[i].y,c:123  };
    }


}
