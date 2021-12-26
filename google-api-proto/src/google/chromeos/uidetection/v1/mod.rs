/// Request message for UI detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiDetectionRequest {
    /// Required. Required field that represents a PNG image.
    #[prost(bytes = "bytes", tag = "1")]
    pub image_png: ::prost::bytes::Bytes,
    /// Required. Required field that indicates the detection type.
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<DetectionRequest>,
}
/// Detection type specifies what to detect in the image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectionRequest {
    #[prost(oneof = "detection_request::DetectionRequestType", tags = "1, 2, 3")]
    pub detection_request_type: ::core::option::Option<detection_request::DetectionRequestType>,
}
/// Nested message and enum types in `DetectionRequest`.
pub mod detection_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DetectionRequestType {
        /// Detection type for word detection.
        #[prost(message, tag = "1")]
        WordDetectionRequest(super::WordDetectionRequest),
        /// Detection type for text block detection.
        #[prost(message, tag = "2")]
        TextBlockDetectionRequest(super::TextBlockDetectionRequest),
        /// Detection type for custom icon detection.
        #[prost(message, tag = "3")]
        CustomIconDetectionRequest(super::CustomIconDetectionRequest),
    }
}
/// Detection type for word detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordDetectionRequest {
    /// Required. The word to locate in the image.
    #[prost(string, tag = "1")]
    pub word: ::prost::alloc::string::String,
}
/// Detection type for text block detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextBlockDetectionRequest {
    /// Required. The text block consisting a list of words to locate im the image.
    #[prost(string, repeated, tag = "1")]
    pub words: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Detection type for custom icon detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIconDetectionRequest {
    /// Required. Required field that represents an icon in PNG format.
    #[prost(bytes = "bytes", tag = "1")]
    pub icon_png: ::prost::bytes::Bytes,
}
/// Response message for UI detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UiDetectionResponse {
    /// Locations of matching UI elements.
    #[prost(message, repeated, tag = "1")]
    pub bounding_boxes: ::prost::alloc::vec::Vec<BoundingBox>,
}
/// The location of a UI element.
/// A bounding box is reprensented by its top-left point [left, top]
/// and its bottom-right point [right, bottom].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBox {
    /// The text found in the bounding box.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// The y-coordinate of the top-left point.
    #[prost(int32, tag = "2")]
    pub top: i32,
    /// The x-coordinate of the top-left point.
    #[prost(int32, tag = "3")]
    pub left: i32,
    /// The y-coordinate of the bottom-right point.
    #[prost(int32, tag = "4")]
    pub bottom: i32,
    /// The x-coordinate of the bottom-right point.
    #[prost(int32, tag = "5")]
    pub right: i32,
}
#[doc = r" Generated client implementations."]
pub mod ui_detection_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Provides image-based UI detection service."]
    #[derive(Debug, Clone)]
    pub struct UiDetectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UiDetectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UiDetectionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UiDetectionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Runs the detection."]
        pub async fn execute_detection(
            &mut self,
            request: impl tonic::IntoRequest<super::UiDetectionRequest>,
        ) -> Result<tonic::Response<super::UiDetectionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.uidetection.v1.UiDetectionService/ExecuteDetection",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
