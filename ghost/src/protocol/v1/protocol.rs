use crate::protocol::{
    traits::Protocol,
    types::{
        CreateInviteInput,
        CreateMessageInput,
        InviteDetails,
        MessageDetails,
    },
    v1::{invite, message},
};

pub struct V1;

impl Protocol for V1 {
    fn version(&self) -> &'static str {
        "1"
    }

    fn create_invite(
        &self,
        input: &CreateInviteInput<'_>
    ) -> String {
        invite::create(input).unwrap_or_default()
    }

    fn create_message(
        &self,
        input: &CreateMessageInput<'_>
    ) -> String {
        message::create(input).unwrap_or_default()
    }

    fn parse_invite(
        &self,
        invite: &str
    ) -> Result<InviteDetails, String> {
        invite::parse(invite)
    }

    fn parse_message(
        &self,
        message: &str
    ) -> Result<MessageDetails, String> {
        message::parse(message)
    }
}

