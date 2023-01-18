use std::cell::RefCell;
use std::io::{Read, Write};
use clap::{App, ArgMatches};

use crate::rules::errors::Error;

pub trait Command {
    fn name(&self) -> &'static str;
    fn command(&self) -> App<'static, 'static>;
    fn execute(&self, args: &ArgMatches, reader: impl Read, writer: impl Write) -> Result<i32, Error>;
}