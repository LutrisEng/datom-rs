// SPDX-FileCopyrightText: 2022 Lutris, Inc
// SPDX-License-Identifier: BlueOak-1.0.0 OR BSD-2-Clause-Patent
// SPDX-FileContributor: Piper McCorkle <piper@lutris.engineering>

use datom::{backends::RedBlackTreeSetStorage, parse_edn, Connection, DynamicConnection, Fact};
use neon::prelude::*;

struct BoxableConnection(DynamicConnection);

impl Finalize for BoxableConnection {}

struct BoxableFact(Fact);

impl Finalize for BoxableFact {}

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

fn fact_from_edn(mut cx: FunctionContext) -> JsResult<JsBox<BoxableFact>> {
    let arg: Handle<JsString> = cx.argument(0)?;
    let parsed = parse_edn(&arg.value(&mut cx)).expect("Failed to parse edn");
    let fact = Fact::from_edn(parsed);
    Ok(cx.boxed(BoxableFact(fact.expect("Invalid fact edn"))))
}

fn fact_to_edn(mut cx: FunctionContext) -> JsResult<JsString> {
    let arg: Handle<JsBox<BoxableFact>> = cx.argument(0)?;
    Ok(cx.string(arg.0.to_edn()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("version", version)?;
    cx.export_function("new_connection", new_connection)?;
    cx.export_function("connection_latest_t", connection_latest_t)?;
    cx.export_function("fact_from_edn", fact_from_edn)?;
    cx.export_function("fact_to_edn", fact_to_edn)?;
    Ok(())
}
