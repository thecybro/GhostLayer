use crate::protocol::{
    create_invite,
    create_message,
    parse_invite,
    parse_message,

    CreateInviteInput,
    CreateMessageInput,
    InviteDetails,
    MessageDetails
};

pub fn create_invite_key(
    public_key: &str, 
    nickname: Option<&str>
) -> Result<String, String> {
    create_invite(
        &CreateInviteInput {
            public_key,
            nickname,
        }
    )
}

pub fn extract_details_from_invite_key(
    invite_key: &str
) -> Result<InviteDetails, String> {
    
    parse_invite(invite_key)
}


pub fn create_message_key(
     sender_public_key: &str,
     nonce_b64: &str,
     ciphertext_b64: &str, ) -> Result<String, String> {

    create_message(
        &CreateMessageInput{
            sender_public_key,
            nonce_b64,
            ciphertext_b64
        },
    )
}


pub fn extract_details_from_message_key(
    message_key: &str
) -> Result<MessageDetails, String> {
    parse_message(message_key)
}
