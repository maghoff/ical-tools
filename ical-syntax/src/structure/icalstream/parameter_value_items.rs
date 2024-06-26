use crate::structure::ParamValueItem;

/// `name` as defined by the following notation:
///
/// ```abnf
/// name          = iana-token / x-name
///
/// iana-token    = 1*(ALPHA / DIGIT / "-")
/// ; iCalendar identifier registered with IANA
///
/// x-name        = "X-" [vendorid "-"] 1*(ALPHA / DIGIT / "-")
/// ; Reserved for experimental use.
///
/// vendorid      = 3*(ALPHA / DIGIT)
/// ; Vendor identification
/// ```
///
/// Reference: [RFC 5545 3.1](https://www.rfc-editor.org/rfc/rfc5545#section-3.1)
pub struct Name;

impl ParamValueItem for Name {
    const QUOTED: bool = false;
}
