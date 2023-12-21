
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
    return fetch(process.env.REACT_APP_API_URL+relative_url, basic_json_template(method,body));
}

function handle_response(response) {
    console.log(response);
    if (response.ok) {
        return response.text().then(text => {
            const data = text && JSON.parse(text);
            if (data === "") {
                const error = (data && data.message) || response.status_text;
                console.log(error);
                return Promise.reject(error);
            }
        })
    }
    else {
        const error = response.status_text;
        console.log(error);
        return Promise.reject(error);
    }
}
