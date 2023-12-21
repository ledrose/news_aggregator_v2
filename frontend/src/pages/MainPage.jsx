import ErrorComponent from "../components/error_boundary";
import fetch_news from "../components/backend_api/news";
import NewsBlock from "../components/main_page/news_block";
import { useQuery } from "react-query";
import { useEffect, useState } from "react";
import useCustomFetch from "../_helpers/CustomFetchHook";
import InfiniteScroll from "react-infinite-scroll-component"

const prefs = [
    {
        action: "Remove",
        pref_type: {
            type:"Theme",
            name:"Другое"
        }
    }
]

export default function MainPage() {
    const load_at_once = 15
    const [data,setData] = useState([]);
    const [loadNext,setLoadNext] = useState(true)
    const [dateOffset,setDateOffset] = useState(undefined);
    const [isLoading, resp ,error, sendRequest] = useCustomFetch(fetch_news,
        (newData) => {
            setData([
                ...data,
                ...newData
            ]);
            if (newData.length<load_at_once) {
                setLoadNext(false);
            } else {
                setDateOffset(newData[newData.length-1].date_time);
            }
        }
    );
    return <div>
        {data.length>0 && data?.map((el)=> 
            <div key={el.id}>   
                <NewsBlock news_info={el}/>
                <hr></hr>
            </div>
        )}
        {loadNext &&
            <button onClick={()=>loadNext && sendRequest(dateOffset,load_at_once,prefs)}>Load more</button>
        }
        {!loadNext &&
            <p>Scroll Ended</p>
        }
    </div>
}

function LoadingPage() {
    return <div>
        <p>Loading</p>
    </div>
}