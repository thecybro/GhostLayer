/// The format we are aiming for:
/// 
/// ghl:inv:1:<invite-payload>
/// ghl:msg:1:<message-payload>
///
/// Where:
/// - ghl = GhostLayer marker
/// - inv or msg = invite/message kind
/// - 1 = protocol version
/// - remainder = version-specific payload

pub const ROOT_PREFIX: &str = "ghl";

// X25519 public keys are 32 bytes.
// Standard Base64 represents 32 bytes using 44 characters.
pub const PUBLIC_KEY_B64_LENGTH: usize = 44;

// ChaCha20Poly1305 uses a 12-byte nonce.
// Standard Base64 represents 12 bytes using 16 characters.
pub const NONCE_B64_LENGTH: usize = 16;

pub const MESSAGE_FRAME_KIND: &str = "msg";
pub const INVITE_FRAME_KIND: &str = "inv";

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FrameKind{
    Invite,
    Message,
}

pub struct Frame<'a> {
    pub kind: FrameKind,
    pub version: &'a str,
    pub payload: &'a str
}


pub fn create_frame(
    kind: FrameKind,
    version: &str,
    payload: &str,
) -> String { 
    
    let kind = match kind {
        FrameKind::Invite => INVITE_FRAME_KIND,
        FrameKind::Message => MESSAGE_FRAME_KIND,
    };

    // looks like ghl:msg:1:<message-payload>
    format!("{ROOT_PREFIX}:{kind}:{version}:{payload}") 
}



pub fn parse_frame(
    frame: &str
) -> Result<Frame<'_>, String> { // later, we gotta replace Err(String) with a custom GhostLayerError
    // the format would be like ghl:inv:1:<invite-payload>, where delimiter is : so we split by it
    // we dont have to care about the format of payload (which might contain : in itself) because
    // we split only upto the version, and keep the rest as is
    let mut parts = frame.splitn(4, ":"); // returns Option<>

    let prefix = parts.next();
    let kind = parts.next();
    let version = parts.next();
    let payload = parts.next();

    if prefix != Some(ROOT_PREFIX) {
        return Err("Invalid ghostlayer prefix!".into())
    };

    let kind = match kind {
        Some(INVITE_FRAME_KIND) => FrameKind::Invite,
        Some(MESSAGE_FRAME_KIND) => FrameKind::Message,
        _ => {
            return Err("Invalid ghostlayer kind!".into())
        }
    };

    let version = version
        .ok_or_else(|| "Invalid version number!")?; // ok_or_else() to convert Option<T> into a Result<T, E>

    let payload = payload
        .ok_or_else(|| "Invalid payload!")?; // ? to get the value inside Ok() quickly on success, and Err if fails

    Ok(Frame{
        kind,
        version,
        payload
    })
}

