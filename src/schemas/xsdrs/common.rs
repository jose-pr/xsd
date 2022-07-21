use super::*;

#[derive(Debug)]
pub enum Namespace {
    Uri(Str),
    ///Target namespace of the parent element
    TargetNamespace,
    ///Not qualified with a namespace
    Local,
}

#[derive(Debug, Default)]
///Any is the default
pub enum Namespaces {
    #[default]
    ///Any namespace.
    Any,
    ///Any namespace that is not the target namespace of the parent element
    Other,
    ///Attributes from any of the namespaces can be present.
    From(List<Namespace>),
}

#[derive(Debug, Default)]
///An indicator of how an application or XML processor should handle validation, the default is strict
pub enum Validation {
    #[default]
    ///The XML processor must obtain the schema for the required namespaces and validate from those namespaces.
    Strict,
    ///The XML processor attempts to obtain the schema for the required namespaces and validate from those namespaces; however, if the schema cannot be obtained, no errors will occur
    Lax,
    ///The XML processor does not attempt to validate any attributes from the specified namespaces.
    Skip,
}