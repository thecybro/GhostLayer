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
    ) -> Result<String, String> {
        invite::create(input)
    }

    fn create_message(
        &self,
        input: &CreateMessageInput<'_>
    ) -> Result<String, String> {
        message::create(input)
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

