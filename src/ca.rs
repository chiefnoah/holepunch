use crate::config::{APPNAME};
use crate::error::{Error, Result};

const CACOMMONNAME: &str = APPNAME;
const CACOUNTRY: &str = "NET";
// five years
const CAEXPIRY: usize = 1825;




fn ensure_ca() -> Result<()> {
    todo!();
}
