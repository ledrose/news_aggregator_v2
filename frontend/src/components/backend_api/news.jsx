import request from "../../_helpers/FetchHelper";


export default async function fetch_news(maxId, amount,searchQuery) {
    return request("news/batch","POST",{
        "start_date": maxId,
        "amount": amount,
        "prefs": searchQuery
    });
}

export async function get_search_options_api() {
    return request("news/search_info","GET")
}