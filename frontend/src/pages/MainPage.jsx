import ErrorComponent from "../components/error_boundary";
import fetch_news from "../components/backend_api/news";
import NewsBlock from "../components/main_page/news_block";
import { useQuery } from "react-query";
import { useEffect, useState } from "react";

export default function MainPage() {
    return (
        <FetchComponent/>
    )
    
}

function FetchComponent() {
    const [data,setData] = useState([]);
    const [dateOffset,setDateOffset] = useState(undefined);
    const [load,setLoad] = useState(false);
    // console.log(dateOffset);
    useEffect(()=>{
        setLoad(false);
        fetch_news(dateOffset,15).then(
            (newData) => {
                setData([
                    ...data,
                    ...newData
                ]);
                setDateOffset(newData[newData.length-1].date_time);
            }
        )
    },[load]);
    // const {isLoading, isError, data, error} = useQuery('news_batch',() => fetch_news(15,10));
    // if (isLoading) {
    //     return <LoadingPage/>
    // }
    // if (isError) {
    //     return <ErrorComponent error={error}/>
    // }
    return <div>
        {data.length>0 && data?.map((el)=> 
            <div key={el.id}>   
                <NewsBlock news_info={el}/>
                <hr></hr>
            </div>
        )}
        <button onClick={()=>setLoad(true)}>Load more</button>
    </div>
}

function LoadingPage() {
    return <div>
        <p>Loading</p>
    </div>
}