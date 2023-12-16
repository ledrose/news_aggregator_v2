
async function getData() {
    const res = await fetch("http://127.0.0.1:8080/"+"api/news/batch", {
        method: "POST",
        cache: "no-cache",
        credentials: "include",
        headers: {
            "Content-Type": "application/json"
        },
        mode: "cors",
        body: JSON.stringify({
            "max_id": 25, "amount": 15
        })
    });
    if (!res.ok) {
        throw new Error("Failed to fetch data");
    }
    return res.json();
}

export default async function Page() {
    const data = await getData();
    // console.log(data)
    return <h1>{JSON.stringify(data)}</h1>
}