use crate::record::{InputRecord, OutputRecord, Record};
use crate::repository::RepositoryOpt;
use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct RunOpt {
    pub command: String,
    pub args: Vec<String>,

    #[structopt(flatten)]
    pub repository: RepositoryOpt,
}

impl RunOpt {
    pub fn run(&self) -> anyhow::Result<()> {
        let record = self.execute_command()?;
        Ok(())
    }

    fn execute_command(&self) -> anyhow::Result<Record> {
        let mut child = Command::new(&self.command)
            .args(self.args.clone())
            .spawn()?;
        let status = child.wait()?;
        let record = Record {
            input: InputRecord {
                command: self.command.clone(),
                args: self.args.clone(),
            },
            output: OutputRecord {
                exit_status: status.code(),
                signal: status.signal(),
            },
        };
        Ok(record)
    }
}
