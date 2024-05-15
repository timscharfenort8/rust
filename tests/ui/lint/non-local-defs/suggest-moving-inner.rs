//@ check-pass

trait Trait<T> {}

fn main() {
    mod below {
        pub struct Type<T>(T);
    }
    struct InsideMain;

    impl Trait<InsideMain> for &Vec<below::Type<InsideMain>> {}
    //~^ WARN non-local `impl` definition
}
