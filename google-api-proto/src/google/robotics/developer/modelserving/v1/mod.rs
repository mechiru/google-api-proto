/// Tensor message for arbitrary input.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tensor {
    /// Tensors in float flattend to 1d. Reshaping information can be infered from
    /// model attributes or other extra inputs.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// Extra inputs to parameterize the inference.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraInputs {
    /// items\[input_key\] specifies value set for an input_key.
    ///
    /// E.g., the following extra inputs will change input.tempeature to 0.1 in
    /// sampling decode.
    /// items {
    ///    "temperature" : "0.1"
    /// }
    #[prost(btree_map = "string, float", tag = "1")]
    pub items: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        f32,
    >,
    /// tensors\[input_key\] specifies tensors set for an input_key.
    ///
    /// E.g., the following extra inputs will change input.tensors as soft prompt.
    /// tensors {
    ///    "prompt_embeddings" : [0.1, 0.2, 0.3, 0.4]
    /// }
    /// It is invalid for the same key to appear in both items and tensors.
    #[prost(btree_map = "string, message", tag = "2")]
    pub tensors: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        Tensor,
    >,
    /// strings\[input_key\] specifies value in string type set for an input_key.
    /// E.g., the following extra inputs will change input.strings as decoding
    /// constraint.
    /// strings {
    ///    "regex" : "a*b*c*d*e*f*g*h*"
    /// }
    /// It is invalid if the same key has appeared in items and tensors.
    #[prost(btree_map = "string, string", tag = "3")]
    pub strings: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Content chunk used as model input or output.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentChunk {
    /// Optional. Mime type of the data. See
    /// <https://www.iana.org/assignments/media-types/media-types.xhtml> for the full
    /// list. Commonly used types that the models are expected to understand:
    ///    text/plain: Generic text, e.g. user's input for an LLM.
    ///    jpeg: JPEG-encoded images.
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Required. Data.
    #[prost(bytes = "bytes", tag = "2")]
    pub data: ::prost::bytes::Bytes,
}
/// Represents a multimodal data which may be streamed. It is an ordered sequence
/// of chunks, where each chunk has a fixed modality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Content {
    /// Required. Data chunks.
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<ContentChunk>,
}
/// Query prompt.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Prompt {
    /// Optional. Multimodal query (text, images etc).
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Content>,
    /// Optional. Extra parameters passed to the model, e.g. 'temperature'.
    #[prost(message, optional, tag = "2")]
    pub extra_inputs: ::core::option::Option<ExtraInputs>,
    /// Required. Model key.
    #[prost(string, tag = "3")]
    pub model_key: ::prost::alloc::string::String,
}
/// Generated plan.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Resulting plan.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<Content>,
}
/// Request for DoPing method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoPingRequest {}
/// Response for DoPing method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoPingResponse {}
/// Request for GeneratePlan method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePlanRequest {
    /// Required. The structured textual input given to the model as a prompt.
    /// Given a prompt, the model will return a sequence of predicted steps.
    #[prost(message, optional, tag = "1")]
    pub prompt: ::core::option::Option<Prompt>,
}
/// Response for GeneratePlan method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratePlanResponse {
    /// Generated plan.
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<Plan>,
}
/// Generated client implementations.
pub mod model_serving_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// ModelServing provides the API for the go/robotics-developer-api.
    #[derive(Debug, Clone)]
    pub struct ModelServingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ModelServingClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ModelServingClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            ModelServingClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// DoPing provides a simple RPC useful for testing.
        pub async fn do_ping(
            &mut self,
            request: impl tonic::IntoRequest<super::DoPingRequest>,
        ) -> std::result::Result<tonic::Response<super::DoPingResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.robotics.developer.modelserving.v1.ModelServing/DoPing",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.robotics.developer.modelserving.v1.ModelServing",
                        "DoPing",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// GeneratePlan generates a sequence of steps to execute an objective.
        /// Example: "pick up an apple".
        pub async fn generate_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::GeneratePlanRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GeneratePlanResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.robotics.developer.modelserving.v1.ModelServing/GeneratePlan",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.robotics.developer.modelserving.v1.ModelServing",
                        "GeneratePlan",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
