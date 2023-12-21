import ErrorComponent from "../components/error_boundary";
import fetch_news from "../components/backend_api/news";
import NewsBlock from "../components/main_page/news_block";
import { useQuery } from "react-query";
import { useEffect, useState } from "react";
import useCustomFetch from "../_helpers/CustomFetchHook";


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
    const [data,setData] = useState([]);
    const [dateOffset,setDateOffset] = useState(undefined);
    const [isLoading, resp ,error, sendRequest] = useCustomFetch(fetch_news,
        (newData) => {
            setData([
                ...data,
                ...newData
            ]);
            setDateOffset(newData[newData.length-1].date_time);
        }
    );
    const [load,setLoad] = useState(false);
    return <div>
        {data.length>0 && data?.map((el)=> 
            <div key={el.id}>   
                <NewsBlock news_info={el}/>
                <hr></hr>
            </div>
        )}
        <button onClick={()=>sendRequest(dateOffset,15,prefs)}>Load more</button>
    </div>
}

function LoadingPage() {
    return <div>
        <p>Loading</p>
    </div>
}