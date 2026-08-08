use crate::{
    protocol::{
        framing::{self, FrameKind},
        traits::Protocol,
        types::{
            CreateInviteInput,
            CreateMessageInput,
            InviteDetails,
            MessageDetails,
        },
        v1::V1,
    },
};

static V1_PROTOCOL: V1 = V1;

static PROTOCOLS: &[&dyn Protocol] = &[ // the list of the protocols we have
    &V1_PROTOCOL,
];

fn current_protocol() -> &'static dyn Protocol {
    &V1_PROTOCOL
}

fn protocol_by_version(
    version: &str,
) -> Result<&'static dyn Protocol, String> { // later replace String with custom error
    PROTOCOLS
        .iter()
        .copied()
        .find(|p| p.version() == version)
        .ok_or_else(|| format!(
            "This was made with GhostLayer protocol version {version}, which this copy of GhostLayer cannot read. Update GhostLayer to open it."
        ))
}

pub fn create_invite(
    input: &CreateInviteInput<'_>
) -> Result<String, String> { // one string is error here
    let protocol = current_protocol();
    let payload = protocol.create_invite(input)?;

    Ok(framing::create_frame(
        FrameKind::Invite,
        protocol.version(),
        &payload,
    ))
}

pub fn parse_invite(
    input: &str,
) -> Result<InviteDetails, String> {
    let frame = framing::parse_frame(input)?;

    if frame.kind != FrameKind::Invite {
        return Err("That is a GhostLayer message, not an invite key. Paste the invite key your friend copied with Copy Invite.".into())
    };

    let protocol = protocol_by_version(frame.version)?;
    protocol.parse_invite(frame.payload)
}

pub fn create_message(
    input: &CreateMessageInput<'_>
) -> Result<String, String> {
    let protocol = current_protocol();
    let payload = protocol.create_message(input)?;

    Ok(framing::create_frame(
        FrameKind::Message,
        protocol.version(), // version() IS A FUNCTION INSIDE PROTOCOL TRAIT!
        &payload,
    ))
}

pub fn parse_message(
    message: &str,
) -> Result<MessageDetails, String> {
    let frame = framing::parse_frame(message)?;
    
    if frame.kind != FrameKind::Message {
        return Err("That is a GhostLayer invite key, not a message. Add it as a friend in the GhostLayer popup instead.".into())
    };

    let protocol = protocol_by_version(frame.version)?;
    protocol.parse_message(frame.payload)
}
