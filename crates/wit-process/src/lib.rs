#![allow(dead_code, unused_variables)]

use std::process::{ExitStatus, Stdio};

use anyhow::Result;
use tokio::process::{Child, Command};
use wasmtime::component::{HasData, Resource};
use wasmtime_wasi::{
    ResourceTable,
    p2::{
        IoError,
        bindings::io::streams::{InputStream, OutputStream},
    },
};

pub use self::generated::wit::*;

mod generated {
    wasmtime::component::bindgen!({
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

pub struct WitProcessCtx {}

pub struct WitProcess<'a> {
    ctx: &'a WitProcessCtx,
    table: &'a mut ResourceTable,
}

impl<'a> WitProcess<'a> {
    pub fn new(ctx: &'a WitProcessCtx, table: &'a mut ResourceTable) -> Self {
        Self { ctx, table }
    }
}

impl process::process::Host for WitProcess<'_> {}

impl process::process::HostStdio for WitProcess<'_> {
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

impl process::process::HostExitStatus for WitProcess<'_> {
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

impl process::process::HostCommand for WitProcess<'_> {
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

impl process::process::HostChild for WitProcess<'_> {
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

pub fn add_to_linker<T: Send + 'static>(
    l: &mut wasmtime::component::Linker<T>,
    f: fn(&mut T) -> WitProcess<'_>,
) -> Result<()> {
    process::process::add_to_linker::<_, HasWitProcess>(l, f)?;
    Ok(())
}

struct HasWitProcess;

impl HasData for HasWitProcess {
    type Data<'a> = WitProcess<'a>;
}
