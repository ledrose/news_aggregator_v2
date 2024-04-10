import request from "../../_helpers/FetchHelper";

export async function get_channels_api() {
    return request("feed","GET");
}
