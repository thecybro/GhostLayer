/// The format we are aiming for later (see formats below), but initially, we might go with just ghl, and ghl_message
/// to validate it works
/// 
/// ghl:i:1:<invite-payload>
/// ghl:m:1:<message-payload>
///
/// Where:
/// - ghl = GhostLayer marker
/// - i or m = invite/message kind
/// - 1 = protocol version
/// - remainder = version-specific payload

pub const ROOT_PREFIX: &str = "ghl";

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
        FrameKind::Invite => "i",
        FrameKind::Message => "m",
    };

    // looks like ghl:m:1:<message-payload>
    format!("{ROOT_PREFIX}:{kind}:{version}:{payload}") 
}



pub fn parse_frame(
    frame: &str
) -> Result<Frame<'_>, String> { // later, we gotta replace Err(String) with a custom GhostLayerError
    // the format would be like ghl:i:1:<invite-payload>, where delimiter is : so we split by it
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
        Some("i") => FrameKind::Invite,
        Some("m") => FrameKind::Message,
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

