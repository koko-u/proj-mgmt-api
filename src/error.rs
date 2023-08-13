use error_stack::IntoReport;
use error_stack::ResultExt;

#[derive(Debug, derive_more::Display)]
#[display(fmt = "App Error")]
pub struct AppErr;
pub trait IntoAppErr {
    type Ok;
    fn into_apperr(self) -> error_stack::Result<Self::Ok, AppErr>;
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
