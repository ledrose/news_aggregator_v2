import ErrorComponent from "../components/error_boundary";
import fetch_news from "../components/backend_api/news";
import NewsBlock from "../components/main_page/news_block";
import { useEffect, useState } from "react";
import useCustomFetch from "../_helpers/CustomFetchHook";
import { Col, Row, Spinner } from "react-bootstrap";
import {reducer, QueryBlock} from "../components/main_page/QuerySettings/QuerySettings";
import { useReducer } from "react";
const defaultQuery = {
    query: "",
    add_source: [],
    remove_source: [],
    add_themes: [],
    remove_themes: [],
}


export default function MainPage() {
    const load_at_once = 15
    const [data,setData] = useState([]);
    const [loadNext,setLoadNext] = useState(true);
    const [query,dispatchQuery] = useReducer(reducer,defaultQuery);
    const [dateOffset,setDateOffset] = useState(undefined);
    const reset = () => {
        setData([]);
        setLoadNext(true);
        setDateOffset(undefined);
    }
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
    const load = () => sendRequest(dateOffset,load_at_once,query); 
    return <div>
        <QueryBlock dispatchQuery={dispatchQuery} reset={reset}/>
        {data.length>0 && data?.map((el)=> 
            <div key={el.id}>   
                <NewsBlock news_info={el}/>
                <hr></hr>
            </div>
        )}
        {isLoading &&
            <SpinnerLoad/>
        }
        {loadNext &&
            <button onClick={()=>loadNext && load()}>Load more</button>
        }
        {!loadNext &&
            <p>Scroll Ended</p>
        }
    </div>
}


function SpinnerLoad() {
    return (
        <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
        </Spinner>
    );
}