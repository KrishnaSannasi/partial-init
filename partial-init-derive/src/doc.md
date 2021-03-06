Partial Init Derive
---
This crate allows for easy, safe, ergonomic, and panic-free handling of partial 
initialization by providing a zero-cost abstraction for the builder pattern. Handling a 
no_std enviornment just as well as a std enviornment. This crate also takes steps to 
make informative error messages for the users your types!

## Sem-ver changes

**VERY IMPORTANT READ AND UNDERSTAND THROUGLY**

This applies to any type marked by `#[derive(PartialInit)]`

* Adding or removing fields (regardless of visibility) is a **major** breaking change
* Reordering fields (regardless of visibility) is a *minor* breaking change.
    * *Note* this is a balance, users of `PartialInit` would like stability of
        the `Partial*` types, but the author would like the be able to reorder
        fields for a more readable layout
    * To mitigate the effects of this, use type aliases to refer to `Partial*` types.
        That way, there is only one place in where it needs to change if the author
        decides to reorder fields
* Changing the name of a field (regardless of visibility) is a **major** breaking change 
    * unless the old function name is maintained with the `#[func_name]` attribute.

**note** \
The visibility of all types created by this crate is the same as the type it is 
derived for. This means all of the types of your fields are in the same 
visibility as your type. For example, if you apply this derive macro to struct `A`, then
all types created by this macro will have a `pub(crate)` visibility.

```rust
#[derive(PartialInit)]
pub(crate) struct A {
    field: i32,
    other: String
}
```

## Usage

Canonical example (this example will be used to explain things in this 
documentation):
```rust
#[derive(PartialInit)]
pub struct Spell<'a, E: Effect + ?Sized> {
    with_name: String,
    damage: u32,
    range: u32,
    effects: Vec<&'a E>,
}

trait Effect {  }
impl Effect for str { }
impl<'a, E: Effect + ?Sized> Effect for Spell<'a, E> { }

mod defaults {
    pub fn range() -> u32 { 10 }
}
```

This crate introduces the `PartialInit` derive macro. This macro creates a module 
and a bunch of types and impls that define how the you build your type. The most 
important of these types is `Partial{name}` (where `{name}` is the name of the 
struct), this type holds the current state of the build, and is used to verify the 
build process, to make sure that everything is initialized. In the example, the 
corresponding `Parital{name}` for `Spell` is `PartialSpell`. The module is named 
`__{name}__` and holds data about your type, such as what fields it has and some 
type aliases. I will give a more in depth explanation later. Now onwards on how to 
use this crate!

After you have marked your struct with `#[PartialInit]`, you can then use it like 
so.

```rust
let spell: Spell<_> = 
    Spell::uninit()
        .with_name("Fireball".to_owned())
        .damage(20)
        .range(50)
        .effects(vec!["burning", "heat stroke"])
        .build();
```

But this is boring, so I created a macro called `init`, which lives in 
`partial-init-core`. This macro brings back the tradional struct initialization 
for anything that implements `PartialInit`.

```rust
let spell: Spell<_> = init! {
    Spell {
        with_name: "Fireball".to_owned(),
        damage: 20,
        range: 50,
        effects: vec!["burning", "heat stroke"]
    }
}
```

This macro desugars to the boring builder pattern.

## Attributes

Now say we want to change up the field `with_name` to `name` for clarity in some 
other part of code, but we don't want to make a breaking change, by changing the 
public function `name`, which is generated by this macro. Well then you can use 
the `#[func_name]` attribute for fields!

```rust
#[derive(PartialInit)]
pub struct Spell<'a, E: Effect + ?Sized> {
    #[func_name(with_name)]
    name: String,
    damage: u32,
    range: u32,
    effects: Vec<&'a E>,
}
```

This will keep the name of the function the same, but the internal variable name 
won't change! Hooray!!! But now we are lazy, and we don't always have effects for 
our spells, so we want a default, an empty `Vec`. We can tweak the code and get 
this:

```rust
#[derive(PartialInit)]
pub struct Spell<'a, E: Effect + ?Sized> {
    #[func_name(with_name)]
    name: String,
    damage: u32,
    range: u32,

    #[default]
    effects: Vec<&'a E>,
}
```

Now we don't have to specify effects, and if we don't we get use 
`Default::default` to get a default value.

But what if we need a default that is different from `Default::default` or if 
`Default` isn't implemented. Then we can use a function to specify a default.


```rust
#[derive(PartialInit)]
pub struct Spell<'a, E: Effect + ?Sized> {
    #[func_name(with_name)]
    name: String,

    damage: u32,

    #[default(defaults::range)]
    range: u32,

    #[default]
    effects: Vec<&'a E>,
}
```

Finally what if we want to deinitialize a field, for whatever reason, we can then 
add the `#[deinit]` to the struct, and that will allow us to deinialize any field 
we want. Or we can add `#[deinit]` just to the fields we want to be able to 
deinitialize. Note that `#[deinit]` on the struct takes precedence on `#[deinit]` 
on the fields.

```rust
#[derive(PartialInit)]
#[deinit]
pub struct Spell<'a, E: Effect + ?Sized> {
    #[func_name(with_name)]
    name: String,

    damage: u32,

    #[default(defaults::range)]
    range: u32,

    #[default]
    effects: Vec<&'a E>,
}
```

And now we have all the parts! Yay. This final one can be used like so (using the 
boring way):

```rust
let spell: Spell<_> = 
    Spell::uninit()
        .with_name("Fireball".to_owned())
        .damage(20)
        .build();

let spell: Spell<_> = 
    Spell::uninit()
        .with_name("Fireball".to_owned())
        .damage(20)
        .effects(vec!["burning", "heat stroke"])
        .build();

let spell: Spell<_> = 
    Spell::uninit()
        .with_name("Fireball".to_owned())
        .damage(20)
        .range(50)
        .build();
let spell: Spell<_> = 
    Spell::uninit()
        .with_name()
        .damage(20)
        .range(50)
        .deinit_with_name()
        .with_name("Fireball".to_owned())
        .build();
```


Notice that we didn't have to change the `with_name` function at all in the user 
code.

In summurt of the attributes:

`#[func_name]` 

Lets you rename the function name, so you can change the internal name freely.

`#[default]`

Lets you use the `Default` trait to provide a default value.

`#[default(function_path)]`

Lets you use a function to provide a default value.

`#[deinit]`

Lets you deinitialize fields

## Reading error messages

Let's say I forgot to initialize damage, like so

```rust
let spell = Spell::uninit()
                .with_name("Fireball".to_owned())
                .range(50)
                .effects(vec!["burning"])
                .build();
```

Then I will get the following error message

```
error[E0599]: no method named `build` found for type `PartialSpell<'_, str, &str, 
partial_init_core::Uninit<__Spell__::damage, u32>, u32, std::vec::Vec<&str>>` in 
the current scope
--> src\main.rs:135:24
    |
107 | #[derive(PartialInit)]
    |                    - method `build` not found for this
...
135 |                       .build();
    |                        ^^^^^
    |
    = note: the method `build` exists but the following trait bounds were not 
    satisfied:
            `partial_init_core::Uninit<__Spell__::damage, u32> : 
            partial_init_core::Init<__Spell__::damage, u32>`
```

If you see the note at the bottom , we see that 
`partial_init_core::Uninit<__Spell__::damage, u32> : partial_init_core::Init<__Spell__::damage, u32>`
was not satsified. What this means is that we forgot to initialize `damage`, to an 
`u32`. The error message spells it out for us. Yay for hijacking the type system 
to get good error messages!

Now what if we accidentily give damage a `i32` instead of the `u32` it is looking 
for. Then we get:

```
error[E0277]: the trait bound `i32: partial_init_core::Init<__Spell__::damage, 
u32>` is not satisfied
--> src\main.rs:132:24
    |
132 |                       .damage(20i32)
    |                        ^^^^^^ the trait 
    `partial_init_core::Init<__Spell__::damage, u32>` is not implemented for `i32`
```

Which says `'partial_init_core::Init<__Spell__::damage, u32>' is not implemented for 'i32'`
meaning we can't initialize a `u32` with a `i32`.

## A deep dive into the inner workings

This is all of the code (with documentation stripped for brevity) that is 
generated by this macro for the final `Spell` struct. This section will be a 
walkthrough on how it works and why it was created.

Seeing this, we can see the biggest downside to this crate, the longer build 
times. But with longer build times you get better error messages and safer, faster 
code.

```rust
#[allow(non_camel_case_types)]
pub mod __Spell__ {
    pub enum with_name {}
    impl ::partial_init_core::FieldName for with_name {}
    pub enum damage {}
    impl ::partial_init_core::FieldName for damage {}
    pub enum range {}
    impl ::partial_init_core::FieldName for range {}
    pub enum effects {}
    impl ::partial_init_core::FieldName for effects {}
    pub mod uninit {
        use super::super::*;
        pub type with_name = ::partial_init_core::Uninit<super::with_name, String>;
        pub type damage = ::partial_init_core::Uninit<super::damage, u32>;
        pub type range = ::partial_init_core::Uninit<super::range, u32>;
        pub type effects<'a, E> = ::partial_init_core::Uninit<super::effects, Vec<&'a E>>;
    }
}
#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
pub struct PartialSpell<
    'a,
    E: Effect + ?Sized,
    with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
    damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
    range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
    effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
> {
    with_name: with_name,
    damage: damage,
    range: range,
    effects: effects,
    __phantom_data__partial_init_: ::partial_init_core::PhantomData<(String, u32, u32, Vec<&'a E>)>,
}
#[allow(non_camel_case_types)]
impl<'a, E: Effect + ?Sized> ::partial_init_core::PartialInit for Spell<'a, E> {
    type Uninitialized = PartialSpell<
        'a,
        E,
        ::partial_init_core::Uninit<__Spell__::with_name, String>,
        ::partial_init_core::Uninit<__Spell__::damage, u32>,
        ::partial_init_core::Uninit<__Spell__::range, u32>,
        ::partial_init_core::Uninit<__Spell__::effects, Vec<&'a E>>,
    >;
    #[inline(always)]
    fn uninit() -> Self::Uninitialized {
        Default::default()
    }
}
impl<'a, E: Effect + ?Sized> Default
    for PartialSpell<
        'a,
        E,
        ::partial_init_core::Uninit<__Spell__::with_name, String>,
        ::partial_init_core::Uninit<__Spell__::damage, u32>,
        ::partial_init_core::Uninit<__Spell__::range, u32>,
        ::partial_init_core::Uninit<__Spell__::effects, Vec<&'a E>>,
    >
{
    #[inline(always)]
    fn default() -> Self {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: Default::default(),
            damage: Default::default(),
            range: Default::default(),
            effects: Default::default(),
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        with_name: ::partial_init_core::Init<__Spell__::with_name, String>,
        damage: ::partial_init_core::Init<__Spell__::damage, u32>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    > PartialSpell<'a, E, with_name, damage, range, effects>
{
    #[inline(always)]
    pub fn build(self) -> Spell<'a, E> {
        Spell {
            name: ::partial_init_core::Init::get(self.with_name),
            damage: ::partial_init_core::Init::get(self.damage),
            range: ::partial_init_core::MaybeInit::get(self.range).unwrap_or_else(defaults::range),
            effects: ::partial_init_core::MaybeInit::get(self.effects).unwrap_or_default(),
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    >
    PartialSpell<
        'a,
        E,
        ::partial_init_core::Uninit<__Spell__::with_name, String>,
        damage,
        range,
        effects,
    >
{
    #[inline(always)]
    pub fn with_name<with_name: ::partial_init_core::Init<__Spell__::with_name, String>>(
        self,
        with_name: with_name,
    ) -> PartialSpell<'a, E, with_name, damage, range, effects> {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name,
            damage: self.damage,
            range: self.range,
            effects: self.effects,
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    >
    PartialSpell<
        'a,
        E,
        with_name,
        ::partial_init_core::Uninit<__Spell__::damage, u32>,
        range,
        effects,
    >
{
    #[inline(always)]
    pub fn damage<damage: ::partial_init_core::Init<__Spell__::damage, u32>>(
        self,
        damage: damage,
    ) -> PartialSpell<'a, E, with_name, damage, range, effects> {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage,
            range: self.range,
            effects: self.effects,
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
        damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    >
    PartialSpell<
        'a,
        E,
        with_name,
        damage,
        ::partial_init_core::Uninit<__Spell__::range, u32>,
        effects,
    >
{
    #[inline(always)]
    pub fn range<range: ::partial_init_core::Init<__Spell__::range, u32>>(
        self,
        range: range,
    ) -> PartialSpell<'a, E, with_name, damage, range, effects> {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage: self.damage,
            range,
            effects: self.effects,
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
        damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
    >
    PartialSpell<
        'a,
        E,
        with_name,
        damage,
        range,
        ::partial_init_core::Uninit<__Spell__::effects, Vec<&'a E>>,
    >
{
    #[inline(always)]
    pub fn effects<effects: ::partial_init_core::Init<__Spell__::effects, Vec<&'a E>>>(
        self,
        effects: effects,
    ) -> PartialSpell<'a, E, with_name, damage, range, effects> {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage: self.damage,
            range: self.range,
            effects,
        }
    }
}
#[allow(non_camel_case_types)]
impl<
        'a,
        E: Effect + ?Sized,
        with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
        damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    > PartialSpell<'a, E, with_name, damage, range, effects>
{
    #[inline(always)]
    pub fn deinit_with_name(self)
    -> PartialSpell<
        'a,
        E,
        ::partial_init_core::Uninit<__Spell__::with_name, String>,
        damage,
        range,
        effects,
    > {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: Default::default(),
            damage: self.damage,
            range: self.range,
            effects: self.effects,
        }
    }
    #[inline(always)]
    pub fn deinit_damage(self)
    -> PartialSpell<
        'a,
        E,
        with_name,
        ::partial_init_core::Uninit<__Spell__::damage, u32>,
        range,
        effects,
    > {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage: Default::default(),
            range: self.range,
            effects: self.effects,
        }
    }
    #[inline(always)]
    pub fn deinit_range(self)
    -> PartialSpell<
        'a,
        E,
        with_name,
        damage,
        ::partial_init_core::Uninit<__Spell__::range, u32>,
        effects,
    > {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage: self.damage,
            range: Default::default(),
            effects: self.effects,
        }
    }
    #[inline(always)]
    pub fn deinit_effects(self)
    -> PartialSpell<
        'a,
        E,
        with_name,
        damage,
        range,
        ::partial_init_core::Uninit<__Spell__::effects, Vec<&'a E>>,
    > {
        PartialSpell {
            __phantom_data__partial_init_: Default::default(),
            with_name: self.with_name,
            damage: self.damage,
            range: self.range,
            effects: Default::default(),
        }
    }
}
```

---

But before we dive into the inner workings, we need to understand the 
`partial-init-core` crate, so please read the documentation for that here. I will 
not be explaining the traits in `partial-init-core` here.

---

The first thing that is created the module `__Spell__`. This module contains lots 
of information used in error messages. Each of the enums corrosponds to a 
field-initializing-function ( FIF ), and is used to name the FIF in error 
messages. It also holds the `uninit` module, which holds type aliases for 
`partial_init_core::Uninit` for all of the fields on the struct. The `uninit` 
module is helpful for creating your own functions on `Partial{name}` types.

---

Next we create the `PartialSpell` type, which I will copy here, for easy reference.

```rust
#[allow(non_camel_case_types)]
pub struct PartialSpell<
    'a, E: Effect + ?Sized,
    with_name: ::partial_init_core::MaybeInit<__Spell__::with_name, String>,
    damage: ::partial_init_core::MaybeInit<__Spell__::damage, u32>,
    range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
    effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
> {
    with_name: with_name,
    damage: damage,
    range: range,
    effects: effects,
    __phantom_data__partial_init_: ::partial_init_core::PhantomData<(String, u32, u32, Vec<&'a E>)>,
}
```

This is a very generic struct. Each of the fields of `Spell` is a field on 
`PartialSpell` with it's own generic type. There is also a `PhantomData` field, 
that is there so that I can use the generic type on `Spell` everywhere I use 
`ParitalSpell`, and so that I can add the `::partial_init_core::MaybeInit<_, _>` 
bound on the generic types. This bounds enforces the author to be responisble when 
using `PartialSpell`, meaning it can always be built (not precisely true, but good 
enough).

One major benefit of using generics, is that it is a zero-cost abstraction. If you 
only initialize use memory for the fields you initialize. Also, in release mode, 
fully initializing a struct and building it turns into the same assembly as 
initializing the struct directly. (Yay!)

---

Next we see the `::partial_init_core::PartialInit` implementation, this creates 
the uninit function on `Spell`. This allows you to create a safe uninitialized 
`Spell`. This value can then be uninitialzed with FIFs which will be detailed 
later.

We also see a `Default` impl for PartialInit for completeness.

---

Next we see the implementation of the `build` field.
Seen here:
```rust
#[allow(non_camel_case_types)]
impl<
        'a, E: Effect + ?Sized,
        with_name: ::partial_init_core::Init<__Spell__::with_name, String>,
        damage: ::partial_init_core::Init<__Spell__::damage, u32>,
        range: ::partial_init_core::MaybeInit<__Spell__::range, u32>,
        effects: ::partial_init_core::MaybeInit<__Spell__::effects, Vec<&'a E>>,
    > PartialSpell<'a, E, with_name, damage, range, effects>
{
    fn build(self) -> Spell<'a, E> {
        Spell {
            name: ::partial_init_core::Init::get(self.with_name),
            damage: ::partial_init_core::Init::get(self.damage),
            range: ::partial_init_core::MaybeInit::get(self.range).unwrap_or_else(defaults::range),
            effects: ::partial_init_core::MaybeInit::get(self.effects).unwrap_or_default(),
        }
    }
}
```

In the impl-generics we see all of the generic types, you can see that all of the 
fields marked with `#[default]` get a `::partial_init_core::MaybeInit` bound, 
while the rest get a `::partial_init_core::Init` bound. This reflects how default 
values don't have to be initialized by the user, while the other fields do must be 
initialized by the user. In the function we then fetch the value and unwrap it if 
necessary. Note that the default is called lazily, only if the value is not 
supplied by the user.

---

Finally we get to the FIFs, there functions initialize one field of 
`PartialSpell`. All of the other fields get passed through, while the specified 
field gets it's value initialized. Note that the specified value must currently be 
`::partial_init_core::Uninit`, this prevents a value from being initialzed twice.

Along with each FIF, we can see a corrosponding `deinit_{fn_name}`, this is what 
allows deinitialization. We simply replace the field value with an 
`::partial_init_core::Uninit`, which signifies that is is deinitialized.

And this concludes the guided tour, thank you for your time.

## When adding functions to `Partial*`

If you need to make sure a value is uninitialized use, `__{name}__::uninit::{field_name}`
for the corrosponding generic parameter, if you need to make sure that a value is initialized,
use `partial_init_core::Init<__{name}__::{field_name}, {field_type}>` as a trait bound for
a generic type. If it doesn't matter if a value is initialized use `partial_init_core::MaybeInit<__{name}__::{field_name}, {field_type}>` as the trait bound for the generic type. If you need a concrete type, use that type for the corrosponding generic parameter.

When creating a `Partial*`, insert the field initializer `__phantom_data__partial_init_: Default::default()`, or use the `new_partial` macro in `partial_init_core`.

See examples for details.

## Neat tips and tricks

Because all of these implement `partial_init_core::Init<_, T>`
* `fn() -> T`
* `&'a dyn Fn() -> T`
* `Arc<dyn Fn() -> T>`
* `Rc<dyn Fn() -> T>`
* `Box<dyn Fn() -> T>`
* `&'a mut dyn FnMut() -> T`
* `Box<dyn FnMut() -> T>`
* ...

you can lazily initialize arugments using funcitons that return type `T`.

example:

```rust
let spell_name: fn() -> _             =          || "Fireball".to_owned();
let damage    : &mut dyn FnMut() -> _ = &mut     || 20;
let range     : Box<dyn Fn() -> _>    = Box::new(|| 50);
let effects   : &dyn Fn() -> _        = &        || vec!["burning"];
let spell = Spell::uninit()
                .with_name(spell_name)
                .damage(damage)
                .range(range)
                .effects(effects)
                .build();
```

In doing so, expensive calculations can be deferred till build is called.

---

Because `Option<T>: partial_init_core::MaybeInit<_, T>`, you can omit `Some`
when using `Option`. In combination with the `#[default]`, you can get
true optional fields.

example:

```rust
#[derive(PartialInit)]
struct WithOption {
    #[default]
    value: Option<u32>
}

let spell = WithOption::uninit()
                    .value(10)
                    .build();

let spell = WithOption::uninit()
                    .build();
```

---

You can initialize default fields with `Option`.

example:

```rust
#[derive(PartialInit)]
struct WithDefault {
    #[default]
    value: u32
}

let value = if rand_bool() { Some(10) } else { None };

let spell = WithDefault::uninit()
                        .value(value)
                        .build();
```

---

The init macro in partial-init-core can initialize mutiple structs at once, and produces 
a tuple containing each of the values.

```rust
let (
    spell_1,
    spell_2,
    with_option_1,
    with_option_2,
) = init! {
    Spell {
        with_name: "Fireball".to_owned(),
        damage: 30,
        range: 100,
        effects: vec!["burning"]
    },
    Spell {
        with_name: "Frost Touch".to_owned(),
        damage: 0,
        effects: vec!["slow", "frost"]
    },
    WithOption {},
    WithOption {
        value: 10
    }
};
```