import store from "../_store/store"
import {userState} from "../_store/userSlice"
import { selectToken } from "../_store/userSlice";
function basic_json_template(method,body) {
    const token = selectToken(store.getState())
    const params = {
        method: method,
        mode: "cors",
        credentials: "include",
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
export default function request(url,method,body, isProxyFullLink=false) {
    if (!isProxyFullLink) {
        url = process.env.REACT_APP_API_URL+url;
    } else {
        url = process.env.REACT_APP_API_URL+"cors_proxy/"+url;
    }
    if (method == "GET") {
        return fetch(url+"?"+new URLSearchParams(body), basic_json_template(method));
    } else {    
        return fetch(url, basic_json_template(method,body));
    }
}
