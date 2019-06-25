#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}
use std::error::Error;
impl Command {
    pub fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: None,
            args: None,
            env: None,
            current_dir: None,
        }
    }
}
pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}
impl CommandBuilder {
    fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = Some(executable);
        self
    }
    fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = Some(args);
        self
    }
    fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = Some(env);
        self
    }
    fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = Some(current_dir);
        self
    }
    pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
        let executable = match self.executable {
            Some(x) => Ok(x),
            None => Err(Box::<dyn Error>::from(::alloc::fmt::format(
                ::std::fmt::Arguments::new_v1(
                    &["Missing "],
                    &match (&&self.executable,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                    },
                ),
            ))),
        }?;
        let args = match self.args {
            Some(x) => Ok(x),
            None => Err(Box::<dyn Error>::from(::alloc::fmt::format(
                ::std::fmt::Arguments::new_v1(
                    &["Missing "],
                    &match (&&self.args,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                    },
                ),
            ))),
        }?;
        let env = match self.env {
            Some(x) => Ok(x),
            None => Err(Box::<dyn Error>::from(::alloc::fmt::format(
                ::std::fmt::Arguments::new_v1(
                    &["Missing "],
                    &match (&&self.env,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                    },
                ),
            ))),
        }?;
        let current_dir = match self.current_dir {
            Some(x) => Ok(x),
            None => Err(Box::<dyn Error>::from(::alloc::fmt::format(
                ::std::fmt::Arguments::new_v1(
                    &["Missing "],
                    &match (&&self.current_dir,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Debug::fmt)],
                    },
                ),
            ))),
        }?;
        Ok(Command {
            executable,
            args,
            env,
            current_dir,
        })
    }
}
fn main() {}
