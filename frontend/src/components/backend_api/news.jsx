import request from "../../_helpers/FetchHelper";


export default async function fetch_news(maxId, amount,searchQuery) {
    return request("news/batch","POST",{
        "start_date": maxId,
        "amount": amount,
        "prefs": searchQuery
    });
}

