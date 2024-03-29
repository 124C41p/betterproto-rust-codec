use super::{
    error::InteropResult, message::BetterprotoMessage, message_meta::BetterprotoMessageMeta,
};
use crate::descriptors::MessageDescriptor;
use pyo3::{
    intern,
    types::{PyAnyMethods, PyType},
    FromPyObject, Py, Python,
};

#[derive(FromPyObject, Debug)]
pub struct BetterprotoMessageClass(pub(super) Py<PyType>);

impl BetterprotoMessageClass {
    pub fn create_instance<'py>(&self, py: Python<'py>) -> InteropResult<BetterprotoMessage<'py>> {
        Ok(BetterprotoMessage(self.0.bind(py).call0()?))
    }

    pub fn descriptor(&self, py: Python) -> InteropResult<Py<MessageDescriptor>> {
        let rust_codec_attr_name = intern!(py, "_betterproto_rust_codec");

        let cls = self.0.bind(py);
        if let Ok(attr) = cls.getattr(rust_codec_attr_name) {
            if let Ok(descriptor) = attr.downcast::<MessageDescriptor>() {
                return Ok(descriptor.as_unbound().clone());
            }
        }

        let desc = cls
            .call0()?
            .getattr(intern!(py, "_betterproto"))?
            .extract::<BetterprotoMessageMeta>()?
            .into_descriptor()?;
        let descriptor = Py::new(py, desc)?;
        cls.setattr(rust_codec_attr_name, &descriptor)?;
        Ok(descriptor)
    }
}
