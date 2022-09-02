type Foo = () => void;

const a = [];
const b = a;

b.push(1);
console.log(a);

// Enums

enum TSEnum {
  Foo,
  Bar,
  Baz
}
