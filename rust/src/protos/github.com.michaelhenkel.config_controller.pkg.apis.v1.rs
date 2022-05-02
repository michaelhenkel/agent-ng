#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscriptionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(enumeration = "resource::Action", tag = "4")]
    pub action: i32,
}
/// Nested message and enum types in `Resource`.
pub mod resource {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        Add = 0,
        Del = 1,
        Retry = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod config_controller_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ConfigControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ConfigControllerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ConfigControllerClient<T>
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
        ) -> ConfigControllerClient<InterceptedService<T, F>>
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
            ConfigControllerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn subscribe_list_watch(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscriptionRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Resource>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/SubscribeListWatch") ;
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }        pub async fn get_virtual_network (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualNetwork > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualNetwork") ;
            self.inner.unary(request.into_request(), path, codec).await
        }        pub async fn get_virtual_machine_interface (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachineInterface > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachineInterface") ;
            self.inner.unary(request.into_request(), path, codec).await
        }        pub async fn get_virtual_machine (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachine > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachine") ;
            self.inner.unary(request.into_request(), path, codec).await
        }        pub async fn get_virtual_router (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualRouter > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualRouter") ;
            self.inner.unary(request.into_request(), path, codec).await
        }        pub async fn get_routing_instance (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: RoutingInstance > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetRoutingInstance") ;
            self.inner.unary(request.into_request(), path, codec).await
        }        pub async fn get_instance_ip (& mut self , request : impl tonic :: IntoRequest < super :: Resource > ,) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: InstanceIp > , tonic :: Status >{
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetInstanceIP") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod config_controller_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ConfigControllerServer."]
    #[async_trait]
    pub trait ConfigController: Send + Sync + 'static {
        #[doc = "Server streaming response type for the SubscribeListWatch method."]
        type SubscribeListWatchStream: futures_core::Stream<Item = Result<super::Resource, tonic::Status>>
            + Send
            + 'static;
        async fn subscribe_list_watch(
            &self,
            request: tonic::Request<super::SubscriptionRequest>,
        ) -> Result<tonic::Response<Self::SubscribeListWatchStream>, tonic::Status>;
        async fn get_virtual_network (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualNetwork > , tonic :: Status > ;
        async fn get_virtual_machine_interface (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachineInterface > , tonic :: Status > ;
        async fn get_virtual_machine (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachine > , tonic :: Status > ;
        async fn get_virtual_router (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualRouter > , tonic :: Status > ;
        async fn get_routing_instance (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: RoutingInstance > , tonic :: Status > ;
        async fn get_instance_ip (& self , request : tonic :: Request < super :: Resource >) -> Result < tonic :: Response < super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: InstanceIp > , tonic :: Status > ;
    }
    #[derive(Debug)]
    pub struct ConfigControllerServer<T: ConfigController> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ConfigController> ConfigControllerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ConfigControllerServer<T>
    where
        T: ConfigController,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req . uri () . path () { "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/SubscribeListWatch" => { # [allow (non_camel_case_types)] struct SubscribeListWatchSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: ServerStreamingService < super :: SubscriptionRequest > for SubscribeListWatchSvc < T > { type Response = super :: Resource ; type ResponseStream = T :: SubscribeListWatchStream ; type Future = BoxFuture < tonic :: Response < Self :: ResponseStream > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: SubscriptionRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . subscribe_list_watch (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = SubscribeListWatchSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . server_streaming (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualNetwork" => { # [allow (non_camel_case_types)] struct GetVirtualNetworkSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetVirtualNetworkSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualNetwork ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_virtual_network (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetVirtualNetworkSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachineInterface" => { # [allow (non_camel_case_types)] struct GetVirtualMachineInterfaceSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetVirtualMachineInterfaceSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachineInterface ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_virtual_machine_interface (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetVirtualMachineInterfaceSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualMachine" => { # [allow (non_camel_case_types)] struct GetVirtualMachineSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetVirtualMachineSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualMachine ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_virtual_machine (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetVirtualMachineSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetVirtualRouter" => { # [allow (non_camel_case_types)] struct GetVirtualRouterSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetVirtualRouterSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: VirtualRouter ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_virtual_router (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetVirtualRouterSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetRoutingInstance" => { # [allow (non_camel_case_types)] struct GetRoutingInstanceSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetRoutingInstanceSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: RoutingInstance ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_routing_instance (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetRoutingInstanceSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController/GetInstanceIP" => { # [allow (non_camel_case_types)] struct GetInstanceIPSvc < T : ConfigController > (pub Arc < T >) ; impl < T : ConfigController > tonic :: server :: UnaryService < super :: Resource > for GetInstanceIPSvc < T > { type Response = super :: super :: super :: super :: super :: super :: super :: super :: ssd_git :: juniper :: net :: contrail :: cn2 :: contrail :: pkg :: apis :: core :: v1alpha1 :: InstanceIp ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: Resource >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_instance_ip (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetInstanceIPSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , }
        }
    }
    impl<T: ConfigController> Clone for ConfigControllerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ConfigController> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ConfigController> tonic::transport::NamedService for ConfigControllerServer<T> {
        const NAME: &'static str =
            "github.com.michaelhenkel.config_controller.pkg.apis.v1.ConfigController";
    }
}
