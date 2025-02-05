use describe::Describe;

fn main() {
  #[derive(Describe)]
  struct MyStruct {
    _my_string: String,
    _my_enum: MyEnum,
    _my_number: i32,
  }

  #[derive(Describe)]
  enum MyEnum {
    MyVariant1,
  }

  let foo = MyStruct {
    _my_string: "Hello".to_string(),
    _my_enum: MyEnum::MyVariant1,
    _my_number: 42,
  };

  let descr = foo.describe();
  assert_eq!(
    descr,
    "MyStruct is a struct with the following fields: _my_string : String, _my_enum : MyEnum, _my_number : i32"
  );
}
