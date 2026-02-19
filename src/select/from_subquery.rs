use sea_orm::{
    sea_query::{Alias, IntoIden, SelectStatement},
    EntityTrait, QuerySelect, Select,
};

#[allow(clippy::wrong_self_convention)]
pub trait FromSubquery {
    fn from_subquery(self, subquery: SelectStatement) -> Self;

    fn from_subquery_as<T: IntoIden>(self, subquery: SelectStatement, alias: T) -> Self;
}

// struct _Select<E>
// where
//     E: EntityTrait,
// {
//     query: SelectStatement,
//     entity: PhantomData<E>,
// }

impl<E> FromSubquery for Select<E>
where
    E: EntityTrait,
{
    fn from_subquery(self, subquery: SelectStatement) -> Self {
        let entity = E::default();
        self.from_subquery_as(subquery, Alias::new(entity.table_name()))
    }

    fn from_subquery_as<T: IntoIden>(mut self, subquery: SelectStatement, alias: T) -> Self {
        // debug_assert_eq!(
        //     std::mem::size_of::<_Select<E>>(),
        //     std::mem::size_of::<Select<E>>()
        // );

        // let mut r = unsafe { std::mem::transmute::<Select<E>, _Select<E>>(self) };

        self.query().from_clear().from_subquery(subquery, alias);

        // unsafe { std::mem::transmute::<_Select<E>, Select<E>>(r) }

        self
    }
}
