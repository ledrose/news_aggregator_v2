import { useEffect, useReducer, useState } from "react";
import { Button, Form, FormGroup, Row,Col } from "react-bootstrap";
import "./QuerySettings.css"
import useCustomFetch from "../../../_helpers/CustomFetchHook";
import { get_search_options_api } from "../../backend_api/news";

const defaultQuery = {
    query: "",
    add_source: [],
    remove_source: [],
    add_themes: [],
    remove_themes: [],
}
function removeItem(arr,value) {
    var index = arr.indexOf(value);
    if (index > -1) {
        arr.splice(index,1);
    }
    return arr
}

export  function reducer(current,action) {
    switch (action.type) {
        case 'query': 
            return {...current,query:action.value};
        case 'source':
            switch (action.state) {
                case 0:
                    return {...current,remove_source:removeItem(current.remove_source,action.label)};
                case 1:
                    return {...current,add_source:[...current.add_source,action.label]}
                case 2:
                    return {...current,add_source:removeItem(current.add_source,action.label),remove_source:[...current.remove_source,action.label]};
                
            }
        case 'theme':
            switch (action.state) {
                case 0:
                    return {...current,remove_themes:removeItem(current.remove_themes,action.label)};
                case 1:
                    return {...current,add_themes:[...current.add_themes,action.label]}
                case 2:
                    return {...current,add_themes:removeItem(current.add_themes,action.label),remove_themes:[...current.remove_themes,action.label]};
                
            }
    }
    return current
}

export function QueryBlock({dispatchQuery,reset}) {
    const [,data,,sendRequest] = useCustomFetch(get_search_options_api);
    useEffect(() => sendRequest(),[]);
    const sources = data?.sources??[];
    const themes = data?.themes??[];
    const apply = (state,type,label) => {
        dispatchQuery({type:type,state:state,label:label})
    }
    const apply_text = (el) => {
        dispatchQuery({type:"query",value:el.target.value});
    }
    const onSubmit = (ev) => {
        // console.log(ev);
        ev.preventDefault();
        reset();
    }
    return <Form onSubmit={onSubmit}>
            <Form.Group>
                <Form.Label>Поиск по заголовку</Form.Label>
                <Form.Control onKeyUp={apply_text} type="text" name="query"></Form.Control>
            </Form.Group>
            <div className="mb-3 pt-3">
                <Row className="justify-content-start">
                    {/* <Col md="1"> */}
                        <Form.Label>Источники:</Form.Label>
                    {/* </Col> */}
                {sources.map((el,i) => (
                    <CustomFormCheck key={i} id={i} label={el} apply={apply} type="source"/>
                ))}
                </Row>
            </div>
            <div className="mb-3">
                <Row className="justify-content-start">
                {/* <Col md="1"> */}
                    <Form.Label>Темы:</Form.Label>
                {/* </Col> */}
                {themes.map((el,i) => (
                    <CustomFormCheck key={i} id={i} label={el} apply={apply} type="theme"/>
                ))}
                </Row>
            </div>
            <Row className="justify-content-center">
                {/* <Col> */}
                    <Button size="lg" variant="primary" type="Submit">Найти</Button>
                {/* </Col> */}
            </Row>
        </Form>
}

function CustomFormCheck({label,type,apply}) {
    const [state,setState] = useState(0);
    const changeState = (el) => {
        const newState = (state+1)%3;
        setState(newState);
        apply(newState,type,label);
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