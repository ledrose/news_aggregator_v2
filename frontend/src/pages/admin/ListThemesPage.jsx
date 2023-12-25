import { Button, Container, Row, Col, Table, Form, FormSelect, FormControl } from "react-bootstrap"
import useCustomFetch from "../../_helpers/CustomFetchHook"
import { get_sources_api, get_themes_api, update_source_api, update_themes_api } from "../../components/backend_api/admin";
import { get_search_options_api } from "../../components/backend_api/news";
import { useEffect, useRef, useState } from "react";
import Select from "react-select"
import CreatableSelect from "react-select/creatable";
import TableLayout from "../../Layouts/TableLayout/TableLayout";
// import {select} from "select2"

export default function ListThemesPage() {
    const defaultSource = "Lenta";
    const defaultTheme = "Другие";
    const amount_on_page = 10;
    const [firstId,setFirstId] = useState(0);
    const [themes,setThemes] = useState(new Map());
    const [themesChanged,setThemesChanged] = useState(new Map());
    const [,dataOptions,,sendOptionsRequest] = useCustomFetch(get_search_options_api);
    const [isLoading,data,error,sendRequest] = useCustomFetch(get_themes_api,(data) => {
        if (data.length!=0) {
            setFirstId(data[0].id)
            const map1 = new Map();
            const map2 = new Map();
            data.forEach((el) => {map1.set(el.id,{id:el.id,source:el.source,theme:el.theme,name:el.name,changed:null})});
            data.forEach((el) => {map2.set(el.id,{id:el.id,source:el.source,theme:el.theme,name:el.name,changed:null})});
            setThemes(map1);
            setThemesChanged(map2);
        }
    });
    const [,,,updateRequest] = useCustomFetch(update_themes_api,()=>{
        sendRequest(firstId-amount_on_page,amount_on_page);
        sendOptionsRequest();
    })
    const update_all = () => {
        const toSend = [...themesChanged.values()].filter((el) => el.changed!=null);
        if (toSend.length!=0) {          
            const toSendFinal = toSend.map((el) => ({id:el.id,theme:el.theme}));  
            console.log(toSendFinal);
            updateRequest(toSendFinal);
        }   
    }
    useEffect(() => {
        sendRequest(1,amount_on_page);
        sendOptionsRequest();
    },[])
    const nextPage = () => {
        sendRequest(firstId-amount_on_page,amount_on_page);
    }
    const prevPage = () => {
        sendRequest(firstId+amount_on_page,amount_on_page);
    }
    const updateOld = (key,type,ev) => {
        if (ev!=null) {
            // console.log(ev);
            const entry = themesChanged.get(key);
            if (type=="source") {
                entry.source = ev.value;
            }
            if (type=="theme") {
                entry.theme = ev.value;
            }
                entry.changed = null;
            if (themes.get(key).source==entry.source && themes.get(key).theme==entry.theme) {
                entry.changed = null;
            } else {
                entry.changed = "Updated";
            }
            themesChanged.set(key,entry)
            setThemesChanged(new Map(themesChanged));
        }
    }
    const nextDis = amount_on_page>themes.size;
    const prevDis = firstId===1;
    return (
        <TableLayout>
            <Container>
                <Table striped bordered hover>
                    <thead>
                        <tr>
                            <th className="col-2">#</th>
                            <th className="col-3">Название</th>
                            <th className="col-3">Источник</th>
                            <th className="col-4">Отображаемая тема</th>
                        </tr>
                    </thead>
                    <tbody>
                        {dataOptions!=null && [...themesChanged].map(([key,el]) => 
                            <TableRow key={key} data={el} dataOptions={dataOptions} updateEl={updateOld}></TableRow>
                        )}
                    </tbody>
                </Table>
                <Row className="justify-content-between mb-2">
                    <Col md="3">
                        <Button className="mx-4" disabled={prevDis} onClick={nextPage}>Предыдущая страница</Button>
                    </Col>
                    <Col md="3">
                        <Button className="mx-4"  disabled={nextDis} onClick={prevPage}>Следующая страница</Button>
                    </Col>
                    <Col md="3">
                        <Button className="mx-4"  onClick={update_all} variant="success">Сохранить изменения</Button>
                    </Col>
                </Row>
            </Container>
        </TableLayout>
    )
}

function TableRow({data,dataOptions,updateEl}) {
    const options = dataOptions.themes.map((el) => ({value:el,label:el}));
    const marker = data.changed!=null?data.changed:"";
    return (
        <tr>
            <td>{data.id+" "+marker}</td>
            <td>{data.name}</td>
            <td>{data.source}</td>
            <td>
                <CreatableSelect isClearable options={options} onChange={(ev)=>updateEl(data.id,"theme",ev)} value={{value:data.theme,label:data.theme}}/>
            </td>
        </tr>
    );
}