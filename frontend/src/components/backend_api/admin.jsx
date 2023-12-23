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
