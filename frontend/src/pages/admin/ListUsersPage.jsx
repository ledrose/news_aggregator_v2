import { Button, Container, Row, Col, Table, Form } from "react-bootstrap"
import useCustomFetch from "../../_helpers/CustomFetchHook"
import { get_all_roles_api, get_sources_api, get_users_api, update_source_api, update_users_api } from "../../components/backend_api/admin";
import { useEffect, useRef, useState } from "react";
import TableLayout from "../../Layouts/TableLayout/TableLayout";


export default function ListUsersPage() {
    const amount_on_page = 15;
    const [firstId,setFirstId] = useState(0);
    const [users,setUsers] = useState(new Map());
    const [usersChanged,setUsersChanged] = useState(new Map());
    const [isLoading,data,error,sendRequest] = useCustomFetch(get_users_api,(data) => {
        if (data.length!=0) {
            setFirstId(data[0].id)
            const map1 = new Map();
            const map2 = new Map();
            data.forEach((el) => map1.set(el.id,{id:el.id,email:el.email,role:el.role,changed:null}));
            data.forEach((el) => map2.set(el.id,{id:el.id,email:el.email,role:el.role,changed:null}));
            setUsers(map1);
            setUsersChanged(map2);
        }
    });
    const [,dataOptions,,updateDataOptions] = useCustomFetch(get_all_roles_api);
    const [,,,updateRequest] = useCustomFetch(update_users_api,()=>{
        sendRequest(firstId,amount_on_page);
    })
    const update_all = () => {
        const toSend = [...usersChanged.values()].filter((el) => el.changed!=null);
        if (toSend.length!=0) { 
            const toSendFinal = toSend.map((el) => ({id:el.id,role:el.role,changed:el.changed}))           
            console.log(toSendFinal);
            updateRequest(toSendFinal);
        }   
    }
    useEffect(() => {
        sendRequest(firstId-amount_on_page,amount_on_page);
        updateDataOptions()
    },[])
    const nextPage = () => {
        sendRequest(firstId-amount_on_page,amount_on_page);
    }
    const prevPage = () => {
        sendRequest(firstId+amount_on_page,amount_on_page);
    }
    const updateOld = (key,type,ev) => {
        const entry = usersChanged.get(key);
        if (type=="role") {
            entry.role = ev.target.value;
        }
        if (entry.changed!="Deleted") {
            entry.changed = null;
            if (users.get(key).role==entry.role) {
                entry.changed = null;
            } else {
                entry.changed = "Updated";
            }
        }
        usersChanged.set(key,entry)
        setUsersChanged(new Map(usersChanged));
    }
    const deleteOld = (key) => {
        usersChanged.get(key).changed = "Deleted";
        setUsersChanged(new Map(usersChanged));
    }
    const nextDis = amount_on_page>users.size;
    const prevDis = firstId<amount_on_page;
    return (
        <TableLayout>
            <Container>
                <Table striped bordered hover>
                    <thead>
                        <tr>
                            <th className="col-1">#</th>
                            <th>Почта</th>
                            <th>Роль</th>
                            <th className="col-1"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {dataOptions!=null && [...usersChanged].map(([key,el]) => 
                            <TableRow key={key} data={el} dataOptions={dataOptions} updateEl={updateOld} deleteEl={deleteOld}></TableRow>
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

function TableRow({data,dataOptions,updateEl,deleteEl}) {
    const marker = data.changed!=null?data.changed:"";
    return (
        <tr>
            <td>{data.id+" "+marker}</td>
            <td>{data.email}</td>
            <td>
                <Form.Select onChange={(ev)=>updateEl(data.id,"role",ev)} className="form-control" defaultValue={data.role}>
                    {dataOptions!=null && dataOptions.map((el) => (
                        <option key={el.id} value={el.name}>{el.name}</option>
                    ))}
                </Form.Select>
            </td>
            <td>
                <Button onClick={()=> deleteEl(data.id)} variant="danger">Delete</Button>
            </td>
        </tr>
    );
}