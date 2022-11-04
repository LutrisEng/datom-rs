// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{backends::RedBlackTreeSetStorage, Connection, DynamicConnection};
use neon::prelude::*;

struct BoxableConnection(DynamicConnection);

impl Finalize for BoxableConnection {}

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn version(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(datom::version()))
}

fn new_connection(mut cx: FunctionContext) -> JsResult<JsBox<BoxableConnection>> {
    Ok(cx.boxed(BoxableConnection(Connection::new(Box::new(
        RedBlackTreeSetStorage::new(),
    )))))
}

fn connection_latest_t(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let arg: Handle<JsBox<BoxableConnection>> = cx.argument(0)?;
    Ok(cx.number(arg.0.latest_t().expect("Failed to get latest t") as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("version", version)?;
    cx.export_function("new_connection", new_connection)?;
    cx.export_function("connection_latest_t", connection_latest_t)?;
    Ok(())
}
