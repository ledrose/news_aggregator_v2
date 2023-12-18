import ErrorComponent from "../components/error_boundary";
import fetch_news from "../components/backend_api/news";
import { useQuery } from "react-query";

export default function MainPage() {
    return (
        <FetchComponent/>
    )
    
}

function FetchComponent() {
    const {isLoading, isError, data, error} = useQuery('news_batch',() => fetch_news(15,10));
    if (isLoading) {
        return <LoadingPage/>
    }
    if (isError) {
        return <ErrorComponent error={error}/>
    }
    console.log("Data:"+data);
    return <div>
        {JSON.stringify(data)}
    </div>
}

function LoadingPage() {
    return <div>
        <p>Loading</p>
    </div>
}