export default function NewsBlock({news_info}) {
    return (
        <div id={news_info.id}>
            <a href={news_info.link}>
                <h1>{news_info.header} {(new Date(news_info.date_time)).toLocaleString()}</h1>
            </a>
           
            <h2>Source: {news_info.source}</h2>
            <h3>Theme: {news_info.theme}</h3>
            {news_info.hasOwnProperty("description") && 
                <p>{news_info.description}</p>
            }
        </div>
    )
}