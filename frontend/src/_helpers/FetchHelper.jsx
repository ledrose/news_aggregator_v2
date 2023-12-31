
function basic_json_template(method,body) {
    const params = {
        method: method,
        credentials: "include",
        mode: "cors",
        headers: {
            "Content-type": "application/json"
        }
    };
    if (body!==null) {
        params.body = JSON.stringify(body);
    }
    return params;
}   
export default function request(relative_url,method,body) {
    if (method == "GET") {
        return fetch(process.env.REACT_APP_API_URL+relative_url+"?"+new URLSearchParams(body), basic_json_template(method));
    } else {
        return fetch(process.env.REACT_APP_API_URL+relative_url, basic_json_template(method,body));
    }
}
