const a: number[] = [];
const b = a;

b.push(1);

console.log(a)

enum TSEnum {
    Foo,
    Bar,
    Baz
}

type Foo = {
    bar?: string;
}

const doSomething = (foo: Foo): string | undefined => {
    if(foo.bar) {
        return "yes"
    }
}