import { useEffect, useReducer, useState } from "react";
import { Button, Form, FormGroup, Row,Col } from "react-bootstrap";
import "./QuerySettings.css"
import useCustomFetch from "../../../_helpers/CustomFetchHook";
import { get_search_options_api } from "../../backend_api/news";
import { useDispatch } from "react-redux";
import { add_source, add_theme, remove_source, remove_theme, reset_source, reset_theme, set_query } from "../../../_store/querySlice";
import { useLocation, useNavigate } from "react-router-dom";



export function QueryBlock({reset}) {
    const [,data,,sendRequest] = useCustomFetch(get_search_options_api);
    useEffect(() => sendRequest(),[]);
    const dispatch = useDispatch();
    const location = useLocation();
    const navigate = useNavigate();
    const sources = data?.sources??[];
    const themes = data?.themes??[];
    const onSubmit = (ev) => {
        ev.preventDefault();
        if (location.pathname=="/") {
            reset();
        } else {
            navigate("/");
        }
    }
    return <Form onSubmit={onSubmit}>
            <Form.Group>
                <Form.Label>Поиск по заголовку</Form.Label>
                <Form.Control onKeyUp={(e) => {dispatch(set_query(e.target.value))}} type="text" name="query"></Form.Control>
            </Form.Group>
            <div className="mb-3 pt-3">
                <Row className="justify-content-start">
                        <Form.Label>Источники:</Form.Label>
                {sources.map((el,i) => (
                    <CustomFormCheck key={i} id={i} label={el} dispatch={dispatch} type="source"/>
                ))}
                </Row>
            </div>
            <div className="mb-3">
                <Row className="justify-content-start">
                    <Form.Label>Темы:</Form.Label>
                {themes.map((el,i) => (
                    <CustomFormCheck key={i} id={i} label={el} dispatch={dispatch} type="theme"/>
                ))}
                </Row>
            </div>
            <Row className="justify-content-center">
                <Col md="3">
                    <Button size="lg" variant="primary" type="Submit">Найти</Button>
                </Col>
            </Row>
        </Form>
}

function CustomFormCheck({label,type,dispatch}) {
    const [state,setState] = useState(0);
    const changeState = (_) => {
        const newState = (state+1)%3;
        setState(newState);
        if (type == "source") {
            newState==0?dispatch(reset_source(label)):newState==1?dispatch(add_source(label)):dispatch(remove_source(label));
        } else if (type == "theme") {
            newState==0?dispatch(reset_theme(label)):newState==1?dispatch(add_theme(label)):dispatch(remove_theme(label));
        }
    }
    const checked = (state!==0)?true:false
    const Cname = (state==2)?"form-check-input-crossed":"";
    return <Col>
        <div className="form-check">
            <input type="checkbox" checked={checked} onChange={changeState} className={Cname+" form-check-input"}></input>
            <label title="" className="form-check-label">{label}</label>
        </div>
    </Col>
    
}