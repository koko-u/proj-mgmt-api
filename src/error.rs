use error_stack::IntoReport;
use error_stack::ResultExt;

#[derive(Debug, derive_more::Display)]
#[display(fmt = "App Error")]
pub struct AppErr;
pub trait IntoAppErr {
    type Ok;
    fn into_apperr(self) -> error_stack::Result<Self::Ok, AppErr>;
}
pub trait IntoGraphQLErr {
    type Ok;
    fn into_graphql_err(self) -> std::result::Result<Self::Ok, async_graphql::Error>;
}

impl error_stack::Context for AppErr {}
impl<T, E> IntoAppErr for std::result::Result<T, E>
where
    error_stack::Report<E>: From<E>,
{
    type Ok = T;

    fn into_apperr(self) -> error_stack::Result<T, AppErr> {
        self.into_report().change_context(AppErr)
    }
}

impl<T, C> IntoGraphQLErr for error_stack::Result<T, C>
where
    C: error_stack::Context,
{
    type Ok = T;

    fn into_graphql_err(self) -> std::result::Result<T, async_graphql::Error> {
        self.map_err(|report| {
            log::error!("{report:?}");
            async_graphql::Error::new_with_source(report)
        })
    }
}
