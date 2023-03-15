enum RSEnum {
    Foo(i32),
    Foo2(Option<i32>),
    Bar(String),
    Baz(Vec<String>)
}

enum Option2<T> {
    None,
    Some(T)
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true
        }
    }
}

fn main() {
    // let a = vec![];
    // let mut b = a;

    // b.push(1);

    // println!("{:?}", b);

    // let mut x = 5;
    // {
    //     let y = &mut x;

    //     *y = 7;
    // }
    

    // println!("{:?}", x);

    // let mut n = vec![];
    // let i = &mut n;

    // i.push(1);
    // println!("{:?}", i);


    // let foo = RSEnum::Foo(5);

    // if let RSEnum::Foo(value) = foo {

    // }

    // match foo {
    //     RSEnum::Foo(value) => {

    //     },
    //     RSEnum::Foo2(Some(value)) => {

    //     },
    //     RSEnum::Foo2(None) => {

    //     },
    //     _ => {}
    // }

    // let foo = Some(5);

    // if let Some(value) = foo {
        
    // }
    // match foo {
    //     Some(value) => {

    //     },
    //     None => {

    //     },
    // }

    // foo.map(|x| {

    // });

    // foo.filter(|x| x < &10);

    let foo = Option2::Some(5);

    if foo.is_some() {
        print!("It is some");
    }

}