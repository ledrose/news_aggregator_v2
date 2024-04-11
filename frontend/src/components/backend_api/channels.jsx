import request from "../../_helpers/FetchHelper";

export async function get_channels_api() {
    return request("feed","GET");
}

export async function add_channels_api(channels) {
    return request("feed/add","POST",channels);
}

export async function delete_channels_api(channels) {
    return request("feed/delete","POST",channels);
}

export async function update_channels_api(channels) {
    return request("feed/update","POST",channels);
}

