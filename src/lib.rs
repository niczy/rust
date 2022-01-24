
pub trait PluserTrait {
    fn plus_one(x : i32) -> i32;
}

pub trait PlusTrait {
    fn plus_one(&self, x : i32) -> i32;
}

pub trait SomeTrait: Send + Sync {
    type Pluser: PluserTrait;
}
struct SomeInstance { 
}

struct PluserInstance {
}

impl PluserTrait for PluserInstance {
    fn plus_one(x: i32) -> i32 {
        x+1
    }
}

struct PlusInstance {
}

impl PlusTrait for PlusInstance {
    fn plus_one(&self, x: i32) -> i32 {
        x+1
    }
}

impl SomeTrait for SomeInstance {
    type Pluser = PluserInstance;
}

fn plus_two<T: SomeTrait>(x: i32) -> i32 {
    let y = T::Pluser::plus_one(x);
    T::Pluser::plus_one(y)
}

fn plus_two_v2<T: PluserTrait>(x: i32) -> i32 {
    let y = T::plus_one(x);
    T::plus_one(y)
}

fn plus_two_v3<T: PlusTrait>(x: i32, plus: &T) -> i32 {
    let y = plus.plus_one(x);
    plus.plus_one(y)
}

fn plus_two_v4(x: i32, pluser: &dyn PlusTrait) -> i32 {
    let y = pluser.plus_one(x);
    pluser.plus_one(y)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let x = plus_two::<SomeInstance>(1);
        assert_eq!(x, 3);

        let y = plus_two_v2::<PluserInstance>(1);
        assert_eq!(y, 3);

        let p = &PlusInstance{};
        let z = plus_two_v3(1, p);
        assert_eq!(z, 3);

        let u = plus_two_v4(1, p);
        assert_eq!(u, 3);
    }
}
