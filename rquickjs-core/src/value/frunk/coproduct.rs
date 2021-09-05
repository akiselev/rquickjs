use frunk::coproduct::*;

use crate::{Error, FromJs, IntoJs};

impl<'js> FromJs<'js> for CNil {
    fn from_js(ctx: crate::Ctx<'js>, value: crate::Value<'js>) -> crate::Result<Self> {
        Err(Error::Unknown)
    }
}

impl<'js, HEAD, TAIL> FromJs<'js> for Coproduct<HEAD, TAIL>
where
    HEAD: FromJs<'js>,
    TAIL: FromJs<'js>,
{
    fn from_js(ctx: crate::Ctx<'js>, value: crate::Value<'js>) -> crate::Result<Self> {
        match HEAD::from_js(ctx, value.clone()) {
            Ok(head) => Ok(Self::Inl(head)),
            Err(err) => match TAIL::from_js(ctx, value) {
                Ok(tail) => Ok(Self::Inr(tail)),
                Err(Error::Unknown) => Err(err),
                Err(err) => Err(err),
            },
        }
    }
}

impl<'js, HEAD, TAIL> IntoJs<'js> for Coproduct<HEAD, TAIL>
where
    HEAD: IntoJs<'js>,
    TAIL: IntoJs<'js>,
{
    fn into_js(self, ctx: crate::Ctx<'js>) -> crate::Result<crate::Value<'js>> {
        match self {
            Coproduct::Inl(head) => head.into_js(ctx),
            Coproduct::Inr(tail) => tail.into_js(ctx),
        }
    }
}

impl<'js> IntoJs<'js> for CNil {
    fn into_js(self, _ctx: crate::Ctx<'js>) -> crate::Result<crate::Value<'js>> {
        unreachable!()
    }
}
