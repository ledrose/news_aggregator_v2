import request from "../../_helpers/FetchHelper";

export async function get_sources_api(id0,amount) {
    return request("admin/sources","GET",{
        id0: id0,
        amount: amount
    })
}

export async function update_source_api(sources) {
    return request('admin/sources',"PATCH",sources)
}

export async function get_themes_api(id0,amount) {
    return request("admin/themes","GET",{
        id0: id0,
        amount: amount
    })
}

export async function update_themes_api(themes) {
    return request('admin/themes',"PATCH",themes)
}

export async function get_users_api(id0,amount) {
    return request("admin/users","GET",{
        id0:id0,
        amount:amount
    })
}

export async function get_all_roles_api() {
    return request("admin/roles","GET");
}

export async function update_users_api(data) {
    return request("admin/users","PATCH",data);
}