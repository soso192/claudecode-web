use actix_web::HttpResponse;

/// patch_search 补丁中心后端地址列表（编译期内嵌，随 cc-web.exe 打包，部署后不可修改）。
///
/// 地址在 `src/patch_servers.json` 里配置（include_str! 内嵌），可配多条；
/// 通常用于内网穿透的多个隧道，均指向同一 patch_search 服务。
/// 修改后需重新编译 cc-web（build.bat）才生效。
const PATCH_SERVERS_CONFIG: &str = include_str!("../patch_servers.json");

/// 解析内嵌配置得到去空白的地址列表（保持配置文件中的顺序）。
pub fn patch_search_servers() -> Vec<String> {
    #[derive(serde::Deserialize)]
    struct Config {
        #[serde(default)]
        patch_search_servers: Vec<String>,
    }
    match serde_json::from_str::<Config>(PATCH_SERVERS_CONFIG) {
        Ok(config) => config
            .patch_search_servers
            .into_iter()
            .map(|value| value.trim().trim_end_matches('/').to_string())
            .filter(|value| !value.is_empty())
            .collect(),
        Err(_) => Vec::new(),
    }
}

/// GET /api/patch-config —— 向补丁中心前端下发 patch_search 后端地址列表（代码内写死）。
pub async fn get_patch_config() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "patch_search_servers": patch_search_servers(),
    }))
}
