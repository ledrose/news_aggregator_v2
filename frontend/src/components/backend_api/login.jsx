export async function login_api(email,password) {
    console.log(email,password);
    return fetch(process.env.REACT_APP_API_URL+"auth/login", {
        method: "POST",
        credentials: "include",
        mode: "cors",
        headers: {
            "Content-type": "application/json"
        },
        body: JSON.stringify({
            "email": email,
            "password": password,
        })
    }).then((res)=> res.status);
}

export async function check_login() {
    return fetch(process.env.REACT_APP_API_URL+"admin/users", {
        method: "POST",
        credentials: "include",
        mode: "cors"
    }).then((res)=> res.statusText);
}

export async function register_api(register_form_data) {
    return fetch(process.env.REACT_APP_API_URL+"auth/register", {
        method: "POST",
        credentials: "include",
        mode: "cors",
        data: register_form_data
    }).then((x) => x.json());
}
