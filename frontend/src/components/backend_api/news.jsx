

export default async function fetch_news(max_id, amount) {
    return fetch(process.env.REACT_APP_API_URL+"news/batch", {
        method: "POST",
        credentials: "include",
        mode: "cors",
        headers: {
            "Content-type": "application/json"
        },
        body: JSON.stringify({
            "start_date": max_id,
            "amount": amount,
        })
    }).then((x) => x.json());
}

