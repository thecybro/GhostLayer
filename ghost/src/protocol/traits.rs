use crate::protocol::{
    CreateInviteInput,
    CreateMessageInput,
    InviteDetails,
    MessageDetails
};

pub trait Protocol: Sync {
    fn version(&self) -> &'static str;
 
    fn create_invite(
        &self,
        input: &CreateInviteInput<'_> // anonymous parameter placeholder, gotta learn more about it
    ) -> String ; // later swap String with Result<String, GhostLayer>

    fn create_message(
        &self,
        input: &CreateMessageInput<'_>
    ) -> String ; // later swap String with Result<String, GhostLayer>

    fn parse_invite(
        &self,
        invite: &str,
    ) -> Result<InviteDetails, String>; // later swap with Result<InviteDetails, GhostLayer>

    fn parse_message(
        &self,
        message: &str
    ) -> Result<MessageDetails, String>; // later swap with Result<MessageDetails, GhostLayer>
}

