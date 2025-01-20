use std::{cell::RefCell, str::FromStr};

use kdl::{KdlDocument, KdlError, KdlIdentifier, KdlNode, KdlValue};
use magnus::{
    function, prelude::*, scan_args::scan_args, value::Lazy, Error, ExceptionClass, KwArgs, RArray,
    RClass, RHash, RModule, Ruby, Value,
};

static RSKDL: Lazy<RModule> = Lazy::new(|ruby| ruby.define_module("RsKDL").unwrap());
static RB_KDL: Lazy<RModule> = Lazy::new(|ruby| ruby.class_object().const_get("KDL").unwrap());
static RB_DOCUMENT: Lazy<RClass> =
    Lazy::new(|ruby| ruby.get_inner(&RB_KDL).const_get("Document").unwrap());
static RB_NODE: Lazy<RClass> = Lazy::new(|ruby| ruby.get_inner(&RB_KDL).const_get("Node").unwrap());
static RB_VALUE: Lazy<RClass> =
    Lazy::new(|ruby| ruby.get_inner(&RB_KDL).const_get("Value").unwrap());

static ERROR: Lazy<ExceptionClass> = Lazy::new(|ruby| {
    ruby.get_inner(&RSKDL)
        .define_error("Error", ruby.exception_standard_error())
        .unwrap()
});

type RbResult<T> = Result<T, Error>;

#[magnus::wrap(class = "RsKDL::Error", free_immediately, size)]
struct RbError(RefCell<KdlError>);

fn kdl_error(ruby: &Ruby, error: KdlError) -> Error {
    Error::new(ruby.get_inner(&ERROR), error.to_string())
}

// TODO: handle types
// TODO: handle format?
fn convert_value_to_rb(
    ruby: &Ruby,
    value: &KdlValue,
    ty: Option<&KdlIdentifier>,
) -> RbResult<Value> {
    let rb_value = ruby.get_inner(&RB_VALUE);

    let kw_hash = RHash::new();

    if let Some(ty) = ty {
        kw_hash.aset(ruby.sym_new("type"), ty.value())?;
    }

    match value {
        KdlValue::String(s) => rb_value
            .const_get::<&str, RClass>("String")?
            .new_instance((s.as_str(), KwArgs(kw_hash))),
        KdlValue::Integer(i) => rb_value
            .const_get::<&str, RClass>("Int")?
            .new_instance((*i as i64, KwArgs(kw_hash))),
        KdlValue::Float(f) => rb_value
            .const_get::<&str, RClass>("Float")?
            .new_instance((*f, KwArgs(kw_hash))),
        KdlValue::Bool(b) => rb_value
            .const_get::<&str, RClass>("Boolean")?
            .new_instance((*b, KwArgs(kw_hash))),
        KdlValue::Null => rb_value
            .const_get::<&str, RClass>("NullImpl")?
            .new_instance(((), KwArgs(kw_hash))),
    }
}

fn convert_node_to_rb(ruby: &Ruby, node: &KdlNode) -> RbResult<Value> {
    let rb_node = ruby.get_inner(&RB_NODE);

    let arguments = RArray::new();
    let properties = RHash::new();

    for entry in node.entries() {
        match entry.name() {
            Some(name) => properties.aset(
                name.value(),
                convert_value_to_rb(ruby, entry.value(), entry.ty())?,
            )?,
            None => arguments.push(convert_value_to_rb(ruby, entry.value(), entry.ty())?)?,
        };
    }

    let children = RArray::new();

    if let Some(child_doc) = node.children() {
        for child in child_doc.nodes() {
            children.push(convert_node_to_rb(ruby, child)?)?;
        }
    }

    let kw_hash = RHash::new();

    if let Some(ty) = node.ty() {
        kw_hash.aset(ruby.sym_new("type"), ty.value())?;
    }

    rb_node.new_instance((
        node.name().value(),
        arguments,
        properties,
        children,
        KwArgs(kw_hash),
    ))
}

fn convert_doc_to_rb(ruby: &Ruby, doc: &KdlDocument) -> RbResult<Value> {
    let rb_doc = ruby.get_inner(&RB_DOCUMENT);

    let nodes = RArray::new();

    for node in doc.nodes() {
        nodes.push(convert_node_to_rb(ruby, node)?)?;
    }

    rb_doc.new_instance((nodes,))
}

fn parse(ruby: &Ruby, args: &[Value]) -> RbResult<Value> {
    let args = scan_args::<(String,), (Option<RHash>,), (), (), (), ()>(args)?;
    let (string,) = args.required;
    match KdlDocument::from_str(&string) {
        Ok(doc) => convert_doc_to_rb(ruby, &doc),
        Err(error) => Err(kdl_error(ruby, error)),
    }
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    Lazy::force(&ERROR, ruby);

    let module = ruby.get_inner(&RSKDL);
    module.define_method("parse", function!(parse, -1))?;

    Ok(())
}
