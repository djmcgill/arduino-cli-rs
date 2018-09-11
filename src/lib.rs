use failure::{Error, format_err};
use subprocess::{Popen, PopenConfig, ExitStatus, Redirection};

const BIN_NAME: &'static str = "arduino-cli";

#[cfg(windows)]
const PATH_SEARCH_COMMAND: &'static str = "where";
#[cfg(not(windows))]
const PATH_SEARCH_COMMAND: &'static str = "which";

pub struct InitToken {
    _no_init: ()
}

impl InitToken {
    pub fn initialise() -> Result<InitToken, Error> {
        let config = PopenConfig {
            // swallow stdout
            stdout: Redirection::Pipe, ..Default::default()
        };

        let mut process = Popen::create(&[PATH_SEARCH_COMMAND, BIN_NAME], config)?;
        let exit_status = process.wait()?;

        if let ExitStatus::Exited(0) = exit_status {
            Ok(InitToken{_no_init: ()})
        } else {
            Err(format_err!(
                "Checking for '{}' in the path returned the non-zero exit code: {:?}",
                BIN_NAME,
                exit_status
            ))
        }
    }
}
