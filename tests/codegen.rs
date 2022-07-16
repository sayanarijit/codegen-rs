use codegen_rs::*;

#[test]
fn empty_scope() {
    let scope = Scope::new();

    assert_eq!(scope.to_string(), "");
}

#[test]
fn single_const() {
    let mut scope = Scope::new();

    let const_ = scope.new_const("FOO", "&'static str", r#""bar""#);
    const_.vis("pub");

    let expect = r#"
pub const FOO: &'static str = "bar";"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn single_struct() {
    let mut scope = Scope::new();

    let struct_ = scope.new_struct("Foo");
    struct_.field("one", "usize");
    struct_.field("two", "String");

    let expect = r#"
struct Foo {
    one: usize,
    two: String,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_pushed_field() {
    let mut scope = Scope::new();
    let mut struct_ = Struct::new("Foo");
    let field = Field::new("one", "usize");
    struct_.push_field(field);
    scope.push_struct(struct_);

    let expect = r#"
struct Foo {
    one: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn single_struct_documented_field() {
    let mut scope = Scope::new();

    let doc = vec!["Field's documentation", "Second line"];

    let mut struct_ = Struct::new("Foo");

    let mut field1 = Field::new("one", "usize");
    field1.doc(doc.clone());
    struct_.push_field(field1);

    let mut field2 = Field::new("two", "usize");
    field2.annotation(vec![r#"#[serde(rename = "bar")]"#]);
    struct_.push_field(field2);

    let mut field3 = Field::new("three", "usize");
    field3.doc(doc).annotation(vec![
        r#"#[serde(skip_serializing)]"#,
        r#"#[serde(skip_deserializing)]"#,
    ]);
    struct_.push_field(field3);

    scope.push_struct(struct_);

    let expect = r#"
struct Foo {
    /// Field's documentation
    /// Second line
    one: usize,
    #[serde(rename = "bar")]
    two: usize,
    /// Field's documentation
    /// Second line
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    three: usize,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn single_fn() {
    let mut scope = Scope::new();
    scope
        .new_fn("my_fn")
        .vis("pub")
        .arg("foo", Type::new("uint"))
        .ret(Type::new("uint"))
        .line("let res = foo + 1;")
        .line("res");

    let expect = r#"
pub fn my_fn(foo: uint) -> uint {
    let res = foo + 1;
    res
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn empty_struct() {
    let mut scope = Scope::new();

    scope.new_struct("Foo");

    let expect = r#"
struct Foo;"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn two_structs() {
    let mut scope = Scope::new();

    let mut struct_ = scope.new_struct("Foo");

    struct_.field("one", "usize");
    struct_.field("two", "String");

    scope.new_struct("Bar").field("hello", "World");

    let expect = r#"
struct Foo {
    one: usize,
    two: String,
}

struct Bar {
    hello: World,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_derive() {
    let mut scope = Scope::new();

    let mut struct_ = scope.new_struct("Foo");
    struct_.derive("Debug").derive("Clone");
    struct_.field("one", "usize");
    struct_.field("two", "String");

    let expect = r#"
#[derive(Debug, Clone)]
struct Foo {
    one: usize,
    two: String,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_repr() {
    let mut scope = Scope::new();

    let struct_ = scope.new_struct("Foo").repr("C");
    struct_.field("one", "u8");
    struct_.field("two", "u8");

    let expect = r#"
#[repr(C)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_allow() {
    let mut scope = Scope::new();

    let mut struct_ = scope.new_struct("Foo").allow("dead_code");
    struct_.field("one", "u8");
    struct_.field("two", "u8");

    let expect = r#"
#[allow(dead_code)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_1() {
    let mut scope = Scope::new();
    let struct_ = scope.new_struct("Foo").generic("T").generic("U");
    struct_.field("one", "T");
    struct_.field("two", "U");

    let expect = r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_2() {
    let mut scope = Scope::new();

    let struct_ = scope.new_struct("Foo").generic("T, U");
    struct_.field("one", "T");
    struct_.field("two", "U");

    let expect = r#"
struct Foo<T, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_generics_3() {
    let mut scope = Scope::new();

    let struct_ = scope.new_struct("Foo").generic("T: Win, U");
    struct_.field("one", "T");
    struct_.field("two", "U");

    let expect = r#"
struct Foo<T: Win, U> {
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_1() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .generic("T")
        .bound("T", "Foo")
        .field("one", "T");

    let expect = r#"
struct Foo<T>
where T: Foo,
{
    one: T,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_where_clause_2() {
    let mut scope = Scope::new();

    let struct_ = scope
        .new_struct("Foo")
        .generic("T, U")
        .bound("T", "Foo")
        .bound("U", "Baz");
    struct_.field("one", "T");
    struct_.field("two", "U");

    let expect = r#"
struct Foo<T, U>
where T: Foo,
      U: Baz,
{
    one: T,
    two: U,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_doc() {
    let mut scope = Scope::new();

    scope
        .new_struct("Foo")
        .doc(
            "Hello, this is a doc string\n\
              that continues on another line.",
        )
        .field("one", "T");

    let expect = r#"
/// Hello, this is a doc string
/// that continues on another line.
struct Foo {
    one: T,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_in_mod() {
    let mut scope = Scope::new();

    {
        let module = scope.new_module("foo");
        let struct_ = module
            .new_struct("Foo")
            .doc("Hello some docs")
            .derive("Debug")
            .generic("T, U")
            .bound("T", "SomeBound")
            .bound("U", "SomeOtherBound");
        struct_.field("one", "T");
        struct_.field("two", "U");
    }

    let expect = r#"
mod foo {
    /// Hello some docs
    #[derive(Debug)]
    struct Foo<T, U>
    where T: SomeBound,
          U: SomeOtherBound,
    {
        one: T,
        two: U,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_mod_import() {
    let mut scope = Scope::new();
    scope
        .new_module("foo")
        .import("bar", "Bar")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_repr() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .repr("u8")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[repr(u8)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_allow() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .allow("dead_code")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn scoped_imports() {
    let mut scope = Scope::new();
    let struct_ = scope
        .new_module("foo")
        .import("bar", "Bar")
        .import("bar", "baz::Baz")
        .import("bar::quux", "quuux::Quuuux")
        .new_struct("Foo");
    struct_.field("bar", "Bar");
    struct_.field("baz", "baz::Baz");
    struct_.field("quuuux", "quuux::Quuuux");

    let expect = r#"
mod foo {
    use bar::{Bar, baz};
    use bar::quux::quuux;

    struct Foo {
        bar: Bar,
        baz: baz::Baz,
        quuuux: quuux::Quuuux,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn module_mut() {
    let mut scope = Scope::new();
    scope.new_module("foo").import("bar", "Bar");

    scope
        .get_module_mut("foo")
        .expect("module_mut")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn get_or_new_module() {
    let mut scope = Scope::new();
    assert!(scope.get_module("foo").is_none());

    scope.get_or_new_module("foo").import("bar", "Bar");

    scope
        .get_or_new_module("foo")
        .new_struct("Foo")
        .field("bar", "Bar");

    let expect = r#"
mod foo {
    use bar::Bar;

    struct Foo {
        bar: Bar,
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn function_with_async() {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expect = r#"
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn trait_with_macros() {
    let mut scope = Scope::new();
    let trt = scope.new_trait("Foo");
    trt.r#macro("#[async_trait]");
    trt.r#macro("#[toby_is_cute]");

    let f = trt.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting toby because he is a good boi\");");

    let expect = r#"
#[async_trait]
#[toby_is_cute]
trait Foo {
    async fn pet_toby() {
        println!("petting toby because he is a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn impl_with_macros() {
    let mut scope = Scope::new();
    scope.new_struct("Bar");
    let imp = scope.new_impl("Bar");
    imp.impl_trait("Foo");
    imp.r#macro("#[async_trait]");
    imp.r#macro("#[toby_is_cute]");

    let f = imp.new_fn("pet_toby");
    f.set_async(true);
    f.line("println!(\"petting Toby many times because he is such a good boi\");");

    let expect = r#"
struct Bar;

#[async_trait]
#[toby_is_cute]
impl Foo for Bar {
    async fn pet_toby() {
        println!("petting Toby many times because he is such a good boi");
    }
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_multiple_allow() {
    let mut scope = Scope::new();

    let struct_ = scope
        .new_struct("Foo")
        .allow("dead_code")
        .allow("clippy::all");
    struct_.field("one", "u8");
    struct_.field("two", "u8");

    let expect = r#"
#[allow(dead_code)]
#[allow(clippy::all)]
struct Foo {
    one: u8,
    two: u8,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn enum_with_multiple_allow() {
    let mut scope = Scope::new();

    scope
        .new_enum("IpAddrKind")
        .allow("dead_code")
        .allow("clippy::all")
        .push_variant(Variant::new("V4"))
        .push_variant(Variant::new("V6"));

    let expect = r#"
#[allow(dead_code)]
#[allow(clippy::all)]
enum IpAddrKind {
    V4,
    V6,
}"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}

#[test]
fn struct_with_member_visibility() {
    let mut scope = Scope::new();

    let struct_named = scope.new_struct("Foo");
    let bar = Field::new("foo", "usize").vis("pub").to_owned();

    struct_named.push_field(bar);
    struct_named.field("baz", "i16").vis("pub(crate)");

    scope.new_struct("Bar").tuple_field("Bar").vis("pub");

    let expect = r#"
struct Foo {
    pub foo: usize,
    pub(crate) baz: i16,
}

struct Bar(pub Bar);"#;

    assert_eq!(scope.to_string(), &expect[1..]);
}
