import { Button, Col, Row } from "react-bootstrap";
import {Link} from "react-router-dom";
import "./NewsBlock.css";
import { useEffect, useState } from "react";
import get_article from "../../acticle/article";
export default function NewsBlock({news_info}) {
    const [open,setOpen] = useState(false)
    const col_el = "collapse" + news_info.id;
    return <div id={news_info.id} className="main-news-div">
        <Row className="justify-content-around">
            <Col md="6">
                <p className="news-date">{(new Date(news_info.date_time)).toLocaleString()}</p>
            </Col>
            <Col md="3">
                <p className="news-source">Источник: {news_info.source}</p>
            </Col>
            <Col md="3">
                <p className="news-theme">Тема: {news_info.theme}</p>
            </Col>
        </Row>
        <MainBlock news_info={news_info}/>
        <Row className="justify-content-end">
            <Col md="2">
                <Link to={news_info.link} target="_blank" className="btn btn-primary">Открыть ссылку</Link>
            </Col>
            <Col md="2">
                <button onClick={() => {setOpen(!open)}} className="btn btn-primary" 
                >Открыть сдесь</button>
            </Col>
        </Row>
        {open &&
            <Row className="justify-content-around">
                <Col md="10">
                    <ArticleReadable link={news_info.link} key={news_info.link} />
                </Col>
            </Row>
        }
    </div>
}

function ArticleReadable({link}) {
    const [loaded,setLoaded] = useState(false);
    const [page,setPage] = useState("");
    useEffect(() => {
        get_article(link).then((res) => {
            setPage(res);
            setLoaded(true);
        })
    },[]);
    return <>
        {loaded && <div className="news-text">{page}</div>}
        {!loaded && <div>Loading</div>}
    </>
}

function MainBlock({news_info}) {
    if (news_info.image!=undefined && news_info.image!=null) {
        return <BlockWithPicture news_info={news_info}/>
    }
    if (news_info.description!=undefined && news_info.description!=null) {
        return <BlockWithoutPicture news_info={news_info}/>
    }
    return <BlockWithoutDescription news_info={news_info}/>
}

function BlockWithPicture({news_info}) {
    return <> 
        <Row className="main-news-row">
            <Col md="6">
                <h1 className="news-header">{news_info.header}</h1>
            </Col>
            <Col md="6">
                <img className="news-image" src={news_info.image}></img>
            </Col>
        </Row>
        <Row className="justify-content-center">
            <Col md="11">
            <p className="news-description">{news_info.description}</p>
            </Col>
        </Row>
    </>
    
}

function BlockWithoutPicture({news_info}) {
    return <Row className="main-news-row">
        <Col md="6">
            <h1 className="news-header">{news_info.header}</h1>
        </Col>
        <Col md="6">
            <p className="news-description">{news_info.description}</p>
        </Col>
    </Row>
}

function BlockWithoutDescription({news_info}) {
    return <Row className="main-news-row">
        <Col md="12">
            <h1 className="news-header">{news_info.header}</h1>
        </Col>
    </Row>
}