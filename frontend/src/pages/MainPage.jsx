import fetch_news from "../components/backend_api/news";
import NewsBlock from "../components/main_page/NewsBlock/NewsBlock";
import { Children, useEffect, useRef, useState } from "react";
import useCustomFetch from "../_helpers/CustomFetchHook";
import { Col, Row, Spinner, Container } from "react-bootstrap";
import { useSelector } from "react-redux";
import useInViewport from "../_helpers/UseInViewport";
import ScrollLayout from "../Layouts/ScrollLayout/ScrollLayout";
import { useOutletContext } from "react-router-dom";


export default function MainPage() {
    const load_at_once = 15
    const context = useOutletContext();
    const [queryUpdate,setQueryUpdate] = context.queryHook;
    const [data,setData] = useState([]);
    const divRef = useRef(null);
    const isInViewport = useInViewport(divRef);
    const [loadNext,setLoadNext] = useState(true);
    const query =  useSelector((state) => state.query);
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
    useEffect(() => {
        if (isInViewport && !isLoading && loadNext) {
            sendRequest(dateOffset,load_at_once,query);
        }
    },[isInViewport,loadNext]);
    useEffect(() => {
        setQueryUpdate(false);
        setData([]);
        setLoadNext(true);
        setDateOffset(undefined);
    },[queryUpdate])
    return <ScrollLayout>
        {data.length>0 && data?.map((el)=> 
            <div key={el.id}>   
                <NewsBlock news_info={el}/>
            </div>
        )}
        {isLoading &&
            <SpinnerLoad/>
        }
        {!loadNext &&
            <NewsNotFound/>
        }
        <div ref={divRef}></div>
    </ScrollLayout>
}


function SpinnerLoad() {
    return (
        <Spinner animation="border" role="status">
            <span className="visually-hidden">Loading...</span>
        </Spinner>
    );
}

function NewsNotFound() {
    return <div className="main-news-div">
        <Row className="justify-content-around">
            <Col md="6">
                <p className="news-theme">Новости не найдены</p>
            </Col>
        </Row>
    </div>
}