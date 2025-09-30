#![allow(unused_variables)]
use std::process::{ExitStatus, Stdio};

use tokio::process::{Child, Command};
use wasmtime::component::Resource;
use wasmtime_wasi::p2::{
    IoError,
    bindings::io::streams::{InputStream, OutputStream},
};

use crate::WasmState;

pub use self::generated::wit::*;

mod generated {
    wasmtime::component::bindgen!({
        path: "./wit/v0.1.0-draft",
        world: "wit:process/imports",
        imports: {
            "wit:process/process/[method]command.output": async | trappable,
            "wit:process/process/[method]command.status": async | trappable,
            "wit:process/process/[method]child.kill": async | trappable,
            "wit:process/process/[method]child.wait": async | trappable,
            "wit:process/process/[method]child.wait-with-output": async | trappable,
            default: trappable,
        },
        with: {
            "wasi:io": wasmtime_wasi::p2::bindings::io,
            "wit:process/process/stdio": std::process::Stdio,
            "wit:process/process/exit-status": std::process::ExitStatus,
            "wit:process/process/command": tokio::process::Command,
            "wit:process/process/child": tokio::process::Child,
        },
        trappable_error_type: {
            "wasi:io/error/error" => wasmtime_wasi::p2::IoError
        }
    });
}

impl process::process::Host for WasmState {}

impl process::process::HostStdio for WasmState {
    fn piped(&mut self) -> wasmtime::Result<Resource<Stdio>> {
        todo!()
    }

    fn inherit(&mut self) -> wasmtime::Result<Resource<Stdio>> {
        todo!()
    }

    fn null(&mut self) -> wasmtime::Result<Resource<Stdio>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Stdio>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl process::process::HostExitStatus for WasmState {
    fn success(&mut self, resource: Resource<ExitStatus>) -> wasmtime::Result<bool> {
        todo!()
    }

    fn code(&mut self, resource: Resource<ExitStatus>) -> wasmtime::Result<Option<i32>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<ExitStatus>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl process::process::HostCommand for WasmState {
    fn new(&mut self, program: String) -> wasmtime::Result<Resource<Command>> {
        todo!()
    }

    fn arg(&mut self, resource: Resource<Command>, arg: String) -> wasmtime::Result<()> {
        todo!()
    }

    fn args(&mut self, resource: Resource<Command>, args: Vec<String>) -> wasmtime::Result<()> {
        todo!()
    }

    fn env(
        &mut self,
        resource: Resource<Command>,
        key: String,
        val: String,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn envs(
        &mut self,
        resource: Resource<Command>,
        vars: Vec<(String, String)>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn env_remove(&mut self, resource: Resource<Command>, key: String) -> wasmtime::Result<()> {
        todo!()
    }

    fn env_clear(&mut self, resource: Resource<Command>) -> wasmtime::Result<()> {
        todo!()
    }

    fn current_dir(&mut self, resource: Resource<Command>, dir: String) -> wasmtime::Result<()> {
        todo!()
    }

    fn stdin(&mut self, resource: Resource<Command>, cfg: Resource<Stdio>) -> wasmtime::Result<()> {
        todo!()
    }

    fn stdout(
        &mut self,
        resource: Resource<Command>,
        cfg: Resource<Stdio>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn stderr(
        &mut self,
        resource: Resource<Command>,
        cfg: Resource<Stdio>,
    ) -> wasmtime::Result<()> {
        todo!()
    }

    fn spawn(
        &mut self,
        resource: Resource<Command>,
    ) -> wasmtime::Result<Result<Resource<Child>, Resource<IoError>>> {
        todo!()
    }

    async fn output(
        &mut self,
        resource: Resource<Command>,
    ) -> wasmtime::Result<Result<process::process::Output, Resource<IoError>>> {
        todo!()
    }

    async fn status(
        &mut self,
        resource: Resource<Command>,
    ) -> wasmtime::Result<Result<Resource<ExitStatus>, Resource<IoError>>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Command>) -> wasmtime::Result<()> {
        todo!()
    }
}

impl process::process::HostChild for WasmState {
    fn stdin(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Option<Resource<OutputStream>>> {
        todo!()
    }

    fn stdout(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Option<Resource<InputStream>>> {
        todo!()
    }

    fn stderr(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Option<Resource<InputStream>>> {
        todo!()
    }

    async fn kill(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Result<(), Resource<IoError>>> {
        todo!()
    }

    fn id(&mut self, resource: Resource<Child>) -> wasmtime::Result<Option<u32>> {
        todo!()
    }

    async fn wait(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Result<Resource<ExitStatus>, Resource<IoError>>> {
        todo!()
    }

    fn try_wait(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Result<Option<Resource<ExitStatus>>, Resource<IoError>>> {
        todo!()
    }

    async fn wait_with_output(
        &mut self,
        resource: Resource<Child>,
    ) -> wasmtime::Result<Result<process::process::Output, Resource<IoError>>> {
        todo!()
    }

    fn drop(&mut self, rep: Resource<Child>) -> wasmtime::Result<()> {
        todo!()
    }
}
