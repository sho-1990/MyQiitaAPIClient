use http_util::CONFIG;

pub fn get_items_url(tag: &str, page: &str, per_page: &str) -> String {
    format!("{}/{}/{}/{}?page={}&per_page={}", CONFIG.api.origin, "tags", tag, "items", page, per_page)
}

pub fn get_tags_url() -> String {
    format!("{}/tags/?page=100&per_page=100&sort=name", CONFIG.api.origin)
}