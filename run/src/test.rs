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
    pub fn deinit_with_name(
        self,
    ) -> PartialSpell<
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
    pub fn deinit_damage(
        self,
    ) -> PartialSpell<
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
    pub fn deinit_range(
        self,
    ) -> PartialSpell<
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
    pub fn deinit_effects(
        self,
    ) -> PartialSpell<
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
