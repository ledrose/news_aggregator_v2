import request from "../../_helpers/FetchHelper";
import {Window} from "happy-dom"
import {Readability,isProbablyReaderable} from "@mozilla/readability"

async function get_html(link) {
    return request(link,"GET",{},false)
}

export default async function get_article(link) {
	const resp = await get_html(link);
	if (!resp.ok) {
		return "";
	}
	const window = new Window();
	window.document.write(resp);
	const article = new Readability(window.document).parse();
	return article.content
}