import request from "../../_helpers/FetchHelper";


export default async function fetch_news(max_id, amount,prefs) {
    return request("news/batch","POST",{
        "start_date": max_id,
        "amount": amount,
        "prefs": prefs
    });
}

