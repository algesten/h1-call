use crate::Error;

use super::call::state::{RecvBody, RecvRequest, Send100, SendResponse, WithBody, WithoutBody};
use super::call::Call;

#[derive(Debug)]
pub(crate) enum CallHolder<B> {
    RecvRequest(Call<RecvRequest>),
    Send100(Call<Send100>),
    RecvBody(Call<RecvBody>),
    SendResponse(Call<SendResponse>),
    WithoutBody(Call<WithBody, B>),
    WithBody(Call<WithoutBody, B>),
    Empty,
}

impl CallHolder<()> {
    pub fn new() -> Result<Self, Error> {
        Ok(CallHolder::RecvRequest(Call::new()))
    }

    pub fn as_recv_request_mut(&mut self) -> &mut Call<RecvRequest> {
        match self {
            CallHolder::RecvRequest(v) => v,
            _ => unreachable!(),
        }
    }
}
