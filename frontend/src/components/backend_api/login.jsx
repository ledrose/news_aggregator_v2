import request from "../../_helpers/FetchHelper";
export async function login_api(email,password) {
    // console.log(email,password);
    return request("auth/login","POST",
        {
            "email": email,
            "password": password,
        }
    );
}

export async function logout_api() {
    return request("auth/logout","GET");
}