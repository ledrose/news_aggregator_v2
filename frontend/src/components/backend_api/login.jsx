export async function login_api(email,password) {
    // console.log(email,password);
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
    }).then((res)=> res.json());
}

export async function logout_api() {
    return fetch(process.env.REACT_APP_API_URL+"auth/logout", {
        method: "GET",
        credentials: "include",
        mode: "cors"
    }).then((res)=> res.ok);
}