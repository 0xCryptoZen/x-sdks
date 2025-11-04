🔑 1. 认证功能

OAuth 1.0a 认证

- App Key（Consumer Key）
- App Secret（Consumer Secret）
- Access Token
- Access Token Secret
- OAuth 1.0a 请求签名算法

配置位置： backend/src/config/settings.rs:51-59

// 当前配置结构
TwitterSettings {
app_key: String,
app_secret: String,
access_token: String,
access_secret: String,
}

---

📝 2. 推文发布功能

POST /2/tweets - 发布推文

当前需求： backend/src/integrations/twitter/client.rs:43-58

请求结构：
{
"text": String // 推文内容（最多 280 字符）
}

响应结构：
{
"data": {
"id": String, // 推文 ID
"text": String // 推文内容
}
}

数据存储： backend/src/models/post_history.rs:12-13

- twitter_post_id: 推文 ID
- twitter_url: 推文 URL（格式：https://twitter.com/user/status/{id}）

---

📋 3. SDK 需要实现的核心功能

必需功能（当前项目使用）

1. ✅ OAuth 1.0a 签名 - 请求认证
2. ✅ POST /2/tweets - 发布纯文本推文
3. ✅ 错误处理 - HTTP 状态码、API 错误响应
4. ✅ 响应解析 - JSON 反序列化

建议扩展功能（未来可能需要）

5. ⚪ 媒体上传 - POST /1.1/media/upload
6. ⚪ 带图片发推 - 在 tweets 请求中包含 media_ids
7. ⚪ 速率限制处理 - 429 状态码、重试逻辑
8. ⚪ 推文查询 - GET /2/tweets/:id
9. ⚪ 删除推文 - DELETE /2/tweets/:id

---

🏗️ 建议的 SDK 结构

// X SDK 核心结构
pub struct XClient {
app_key: String,
app_secret: String,
access_token: String,
access_secret: String,
http_client: reqwest::Client,
}

impl XClient {
// 1. 初始化
pub fn new(credentials: OAuth1Credentials) -> Self;

      // 2. 核心功能
      pub async fn post_tweet(&self, request: TweetRequest)
          -> Result<TweetResponse, XError>;

      // 3. OAuth签名（内部使用）
      fn generate_oauth_signature(&self, method: &str, url: &str, params: &HashMap<String, String>)
          -> String;

      // 4. 请求构建（内部使用）
      fn build_authorized_request(&self, method: Method, endpoint: &str, body: &str)
          -> reqwest::Request;

}

// 请求/响应类型 #[derive(Serialize)]
pub struct TweetRequest {
pub text: String,
}

#[derive(Deserialize)]
pub struct TweetResponse {
pub data: TweetData,
}

#[derive(Deserialize)]
pub struct TweetData {
pub id: String,
pub text: String,
}

// 错误类型 #[derive(Debug)]
pub enum XError {
AuthenticationFailed,
RateLimitExceeded,
InvalidRequest(String),
NetworkError(reqwest::Error),
ApiError { code: u16, message: String },
}

---

🔧 技术要求

1. HTTP 客户端：reqwest（已在项目中使用）
2. OAuth 1.0a 库：建议使用 oauth1-request crate
3. 序列化：serde + serde_json
4. 异步运行时：tokio（已在项目中使用）

---

📌 关键 API 端点

Base URL: https://api.twitter.com

| 功能     | 端点              | 方法   | 优先级  |
| -------- | ----------------- | ------ | ------- |
| 发布推文 | /2/tweets         | POST   | 🔴 必需 |
| 上传媒体 | /1.1/media/upload | POST   | 🟡 建议 |
| 查询推文 | /2/tweets/:id     | GET    | 🟢 可选 |
| 删除推文 | /2/tweets/:id     | DELETE | 🟢 可选 |

---

📖 相关文档链接

- https://developer.twitter.com/en/docs/twitter-api
- https://oauth.net/core/1.0a/
- https://developer.twitter.com/en/docs/twitter-api/tweets/manage-tweets/api-reference/post-tweets

---

这个总结涵盖了当前项目最小可行的 X SDK 需求。核心是 OAuth 1.0a 认证和发布推文功能，这两个功能完成后即可满足当前自动发帖机器人的需求。

──
