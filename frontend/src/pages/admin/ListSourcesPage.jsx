import { Button, Container, Table, Form } from "react-bootstrap"
import useCustomFetch from "../../_helpers/CustomFetchHook"
import { get_sources_api, update_source_api } from "../../components/backend_api/admin";
import { useEffect, useRef, useState } from "react";


export default function ListSourcesPage() {
    const amount_on_page = 15;
    const [firstId,setFirstId] = useState(0);
    const [sources,setSources] = useState(new Map());
    const [sourcesChanged,setSourcesChanged] = useState(new Map());
    const [isLoading,data,error,sendRequest] = useCustomFetch(get_sources_api,(data) => {
        if (data.length!=0) {
            setFirstId(data[0].id)
            const map1 = new Map();
            const map2 = new Map();
            data.forEach((el) => map1.set(el.id,{id:el.id,name:el.name,source_type:el.source_type,link:el.link,changed:null}));
            data.forEach((el) => map2.set(el.id,{id:el.id,name:el.name,source_type:el.source_type,link:el.link,changed:null}));
            setSources(map1);
            setSourcesChanged(map2);
        }
    });
    const [,,,updateRequest] = useCustomFetch(update_source_api,()=>{
        sendRequest(firstId,amount_on_page);
    })
    const update_all = () => {
        const toSend = [...sourcesChanged.values()].filter((el) => el.changed!=null);
        if (toSend.length!=0) {            
            console.log(toSend);
            updateRequest(toSend);
        }   
    }
    useEffect(() => {
        sendRequest(0,amount_on_page);
    },[])
    const nextPage = () => {
        sendRequest(firstId-amount_on_page,amount_on_page);
    }
    const prevPage = () => {
        sendRequest(firstId+amount_on_page,amount_on_page);
    }
    const updateOld = (key,type,ev) => {
        const entry = sourcesChanged.get(key);
        if (type=="name") {
            entry.name = ev.target.value;
        }
        if (type=="link") {
            entry.link = ev.target.value;
        }
        if (entry.changed!="Added" && entry.changed!="Deleted") {
            entry.changed = null;
            if (sources.get(key).name==entry.name && sources.get(key).link==entry.link) {
                entry.changed = null;
            } else {
                entry.changed = "Updated";
            }
        }
        sourcesChanged.set(key,entry)
        setSourcesChanged(new Map(sourcesChanged));
    }
    const deleteOld = (key) => {
        if (sourcesChanged.get(key).changed != "Added") {
            sourcesChanged.get(key).changed = "Deleted";
        } else {
            sourcesChanged.delete(key);
        }
        setSourcesChanged(new Map(sourcesChanged));
    }
    const addNew = () => {
        // console.log(Math.max(...sourcesChanged.keys()));
        const newKey = Math.max(...sourcesChanged.keys())+1;
        sourcesChanged.set(newKey,{id:newKey,name:"",source_type:"rss",link:"example.com",changed:"Added"})
        setSourcesChanged(new Map(sourcesChanged));
    }
    // console.log(sourcesChanged);
    const nextDis = amount_on_page>sources.size;
    const prevDis = firstId===1;
    return (
        <Container>
            <Table striped bordered hover>
                <thead>
                    <tr>
                        <th className="col-1">#</th>
                        <th>Название</th>
                        <th>Тип</th>
                        <th>Ссылка</th>
                        <th className="col-3"></th>
                    </tr>
                </thead>
                <tbody>
                    {[...sourcesChanged].map(([key,el]) => 
                        <TableRow key={key} data={el} updateEl={updateOld} deleteEl={deleteOld}></TableRow>
                    )}
                </tbody>
            </Table>
            <Button disabled={prevDis} onClick={nextPage}>Prev</Button>
            <Button disabled={nextDis} onClick={prevPage}>Next</Button>
            <Button onClick={addNew} variant="success">Add New</Button>
            <Button onClick={update_all} variant="success">Update All</Button>
        </Container>
    )
}

function TableRow({data,updateEl,deleteEl}) {
    // console.log(data)
    // const defaultData = data[1];
    // console.log(data);
    // console.log(nameRef.current);
    const marker = data.changed!=null?data.changed:"";
    return (
        <tr>
            <td>{data.id+" "+marker}</td>
            <td>
                <input onChange={(ev)=>updateEl(data.id,"name",ev)} className="form-control" defaultValue={data.name}/>
            </td>
            <td>{data.source_type}</td>
            <td>
                <input onChange={(ev)=>updateEl(data.id,"link",ev)} className="form-control" defaultValue={data.link}/>
            </td>
            <td>
                <Button onClick={()=> deleteEl(data.id)} variant="danger">Delete</Button>
            </td>
        </tr>
    );
}