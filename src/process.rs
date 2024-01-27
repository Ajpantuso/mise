use color_eyre::Result;
use std::ops::{Deref, DerefMut};
use std::process;
use std::process::{Child, Command};
use std::time;

pub fn spawn(cmd: &mut Command) -> Result<Process> {
    Ok(Process {
        proc: cmd.spawn()?,
        instant: time::Instant::now(),
    })
}

pub struct Process {
    proc: Child,
    instant: time::Instant,
}

impl Process {
    pub fn wait(&mut self) -> Result<ExitStatus> {
        let status = self.proc.wait()?;

        Ok(ExitStatus {
            status,
            elapsed: self.instant.elapsed(),
        })
    }
}

impl Deref for Process {
    type Target = Child;

    fn deref(&self) -> &Self::Target {
        &self.proc
    }
}

impl DerefMut for Process {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.proc
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ExitStatus {
    pub status: process::ExitStatus,
    elapsed: time::Duration,
}

impl ExitStatus {
    pub fn elapsed(&self) -> time::Duration {
        self.elapsed
    }
}

impl Deref for ExitStatus {
    type Target = process::ExitStatus;

    fn deref(&self) -> &Self::Target {
        &self.status
    }
}

impl DerefMut for ExitStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.status
    }
}
