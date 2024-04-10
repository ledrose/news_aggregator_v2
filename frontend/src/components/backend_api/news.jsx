import request from "../../_helpers/FetchHelper";


export default async function fetch_news(maxId, amount,searchQuery,allowed_sources) {
    const query = {
        "allowed_sources": allowed_sources,
        "add_source": searchQuery.add_source,
        "remove_source": searchQuery.remove_source,
        "add_themes": searchQuery.add_themes,
        "remove_themes": searchQuery.remove_themes 
    };
    return request("news/batch","POST",{
        "start_date": maxId,
        "amount": amount,
        "prefs": query,
    });
}

export async function get_search_options_api() {
    return request("news/search_info","GET")
}