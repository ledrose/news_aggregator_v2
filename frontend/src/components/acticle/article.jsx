import request from "../../_helpers/FetchHelper";
import {Readability,isProbablyReaderable} from "@mozilla/readability"

async function get_html(link) {
    return request(link,"GET",{},true)
}

export default async function get_article(link) {
	const resp = await get_html(link);
	if (!resp.ok) {
		return "";
	}
	const text = await new Response(resp.body).text();
	const document = new DOMParser().parseFromString(text,"text/html");
	const article = new Readability(document).parse();
	return article.content;
}