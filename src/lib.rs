use std::marker::PhantomData;

// a fancy trait
pub trait CoolTrait {}

// implement that trait for all reference to someone implementing the trait
impl<'a, D: CoolTrait> CoolTrait for &'a D {}

// a type with a lifetime that contains two other types
pub enum Parent<'a> {
    A(Box<A<'a>>),
    B(Box<B<'a>>),
    // We need a lifetime. otherwise, if we remove it, we get:
    // error[E0275]: overflow evaluating the requirement `A: CoolTrait`
    Boo(PhantomData<&'a ()>)
}

// those two types that in turn contain the parent type recursively
pub struct A<'a> {
    parent: Parent<'a>,
}

pub struct B<'a> {
    parent: Parent<'a>,
}

// implement CoolTrait only when the two types themselves implement it
impl<'a> CoolTrait for Parent<'a>
where
    A<'a>: CoolTrait,
    B<'a>: CoolTrait,
{
}

// implement CoolTrait for the two types only when the Parent also implements it
// oh no! a cycle!
impl<'a> CoolTrait for A<'a> where Parent<'a>: CoolTrait {}
impl<'a> CoolTrait for B<'a> where Parent<'a>: CoolTrait {}

// now test whether CoolTrait is implemented for
pub fn foo(parent: Parent<'_>) {
    requires_parent_fulfill_cool_trait(parent);
}

pub fn requires_parent_fulfill_cool_trait(_: impl CoolTrait) {}
