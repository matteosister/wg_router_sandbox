use crate::gql_types::Quote;
use async_graphql::*;

pub type Server2Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    #[graphql(entity)]
    async fn quote(&self, id: ID) -> Quote {
        Quote { id }
    }
}

pub fn get_schema() -> Server2Schema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .finish()
}
