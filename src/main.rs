fn main() {
    let a = vec![];
    let mut b = a;

    b.push(1);
    //a.push(1);

    println!("{:?}", b);

    let mut x = 5;
    {
        let y = &mut x;

        *y = 7;
    }
    

    // x = 5;

    println!("{:?}", x);

    let mut n = vec![];
    let i = &mut n;

    i.push(1);
    println!("{:?}", i);


}