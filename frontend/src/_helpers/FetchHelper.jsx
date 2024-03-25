import store from "../_store/store"
import {userState} from "../_store/userSlice"
import { selectToken } from "../_store/userSlice";
function basic_json_template(method,body) {
    const token = selectToken(store.getState())
    const params = {
        method: method,
        credentials: "include",
        mode: "cors",
        headers: {
            "Content-type": "application/json",
            ...(token==null && {"Authorization": "Bearer "+token})
            
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
