use std::collections::HashMap;

use reqwest::Method;
use serde::Deserialize;

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    SDKResult,
};

use super::MessageService;

/// 获取消息资源响应
#[derive(Debug, Deserialize)]
pub struct GetMessageResourceResp {
    /// 文件名
    pub file_name: String,
    /// 文件内容的二进制数据
    pub data: Vec<u8>,
}

impl ApiResponseTrait for GetMessageResourceResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(Self {
            file_name,
            data: body,
        })
    }
}

impl MessageService {
    /// 获取消息资源
    ///
    /// 获取消息中的资源文件，需要提供消息 ID 和资源的 file_key。
    ///
    /// <https://open.larksuite.com/document/server-docs/im-v1/message/get>
    pub async fn get_resource(
        &self,
        message_id: &str,
        file_type: &str,
        file_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetMessageResourceResp> {
        let query_params = HashMap::from([("type", file_type.to_string())]);
        let api_req = crate::core::api_req::ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                crate::core::endpoints::im::IM_V1_MESSAGE_GET_RESOURCE,
                &[("message_id", message_id), ("file_key", file_key)],
            ),
            query_params,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp: BaseResponse<GetMessageResourceResp> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }
}
