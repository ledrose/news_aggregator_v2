import request from "../../_helpers/FetchHelper";


export default async function fetch_news(max_id, offset, amount,searchQuery,allowed_sources) {
    const query = {
        "allowed_sources": allowed_sources,
        "add_source": searchQuery.add_source,
        "remove_source": searchQuery.remove_source,
        "add_themes": searchQuery.add_themes,
        "remove_themes": searchQuery.remove_themes,
        "start_date": searchQuery.start_date,
        "end_date": searchQuery.end_date,
        "filter": searchQuery.filter,
        "query": searchQuery.query
    };
    return request("news/batch","POST",{
        "max_id": max_id,
        "offset": offset,
        "amount": amount,
        "prefs": query,
    });
}

export async function get_search_options_api() {
    return request("news/search_info","GET")
}