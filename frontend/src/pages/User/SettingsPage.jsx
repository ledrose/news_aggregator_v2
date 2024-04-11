import { setChannel } from "../../_store/userSlice";
import { Button, Container, Col, Table, Form, Row } from "react-bootstrap"
import useCustomFetch from "../../_helpers/CustomFetchHook"
import { useEffect, useRef, useState } from "react";
import TableLayout from "../../Layouts/TableLayout/TableLayout";
import CreatableSelect from "react-select/creatable";
import { useDispatch } from "react-redux";
import { add_channels_api, delete_channels_api, get_channels_api, update_channels_api } from "../../components/backend_api/channels";
import { get_sources_api } from "../../components/backend_api/admin";
export default function RegisterPage() {
    return <TableLayout>
        <Settings/>
    </TableLayout>
}

function Settings() {
    const dispatch = useDispatch();
	const [feeds,setFeeds] = useState(new Map());
	const [sources,setSources] = useState([]);
	const [feedsChanged,setFeedsChanged] = useState(new Map());
    const [,,,getSources] = useCustomFetch(get_sources_api,(data) => setSources(data))
	const [,,,sendChannelGet] = useCustomFetch(get_channels_api,(data) => {
		if (data.length!=0) {
            dispatch(setChannel(data));
            const map1 = new Map();
            const map2 = new Map();
            data.forEach((el,ind) => map1.set(ind,{id: ind, name:el.name, sources:el.sources,changed:null}));
            data.forEach((el,ind) => map2.set(ind,{id: ind, name:el.name, sources:el.sources,changed:null}));
            setFeeds(map1);
            setFeedsChanged(map2);
        }
	});
	const [,,,updateFeed] = useCustomFetch(update_channels_api);
    const [,,,addFeed] = useCustomFetch(add_channels_api);
    const [,,,deleteFeed] = useCustomFetch(delete_channels_api);

    const update_all = () => {
        const toSend = [...feedsChanged.values()].filter((el) => el.changed!=null);
        if (toSend.length!=0) {            
            console.log(toSend);
            Promise.all([
                ...toSend.filter((el) => el.changed=="Added").map((el) => addFeed(el)),
                ...toSend.filter((el) => el.changed=="Updated").map((el) => updateFeed(el)),
                ...toSend.filter((el) => el.changed=="Deleted").map((el) => deleteFeed(el)),
            ]).then((_) => {sendChannelGet()})
        }   
    }
	useEffect(() => {sendChannelGet();getSources(0,10000)},[]);
	const addNew = () => {
        let newKey = Math.max(...feedsChanged.keys())+1;
        if (newKey == -Infinity) {
            newKey = 0;
        }
        feedsChanged.set(newKey,{id:newKey,name:"",sources:[],changed:"Added"})
        setFeedsChanged(new Map(feedsChanged));
	}
    const deleteOld = (key) => {
        if (feedsChanged.get(key).changed != "Added") {
            feedsChanged.get(key).changed = "Deleted";
        } else {
            feedsChanged.delete(key);
        }
        setFeedsChanged(new Map(feedsChanged));
    }
    const updateItem = (key,type,ev) => {
        if (ev!=null) {
            const entry = feedsChanged.get(key);
            if (type=="name") {
                entry.name = ev.target.value;
            }
            if (type=="sources") {
                entry.sources = ev.map((e)=>e.value);
            }
            if (entry.changed!="Added" && entry.changed!="Deleted") {
                entry.changed = null;
                if (feeds.get(key).name==entry.name && feeds.get(key).link==entry.link) {
                    entry.changed = null;
                } else {
                    entry.changed = "Updated";
                }
            }
            feedsChanged.set(key,entry)
            setFeedsChanged(new Map(feedsChanged));
        }
    }
	return <div>
            <Container>
                <Table striped bordered hover>
                    <thead>
                        <tr>
                            <th className="col-1">#</th>
                            <th className="col-2">Название канала</th>
							<th className="col-4">Источники</th>
                            <th className="col-1"></th>
                        </tr>
                    </thead>
                    <tbody>
                        {[...feedsChanged].map(([key,el]) => 
                            <TableRow key={key} data={el} dataOptions={sources} updateEl={updateItem} deleteEl={deleteOld}></TableRow>
                        )}
                    </tbody>
                </Table>
                <Row className="justify-content-between mb-2">
					<Col md="2">
                        <Button className="mx-2" onClick={addNew} variant="success">Добавить строку</Button>
                    </Col>
                    <Col md="3">
                        <Button className="mx-4" onClick={update_all}  variant="success">Сохранить изменения</Button>
                    </Col>
                </Row>
            </Container>
	</div>
}

function TableRow({data,dataOptions,updateEl,deleteEl}) {
    const options = dataOptions.map((el) => ({value:el.name,label:el.name}));
    const marker = data.changed!=null?data.changed:"";
    return (
        <tr>
            <td>{data.id+" "+marker}</td>
            <td>
				<input onChange={(ev)=>updateEl(data.id,"name",ev)} className="form-control" defaultValue={data.name}/>
			</td>
            <td>
                <CreatableSelect isMulti options={options} onChange={(ev)=>updateEl(data.id,"sources",ev)} defaultValue={data.sources.map((el) =>({value:el,label:el}))} />
            </td>
            <td>
                <Button onClick={()=> deleteEl(data.id)} variant="danger">Удалить</Button>
            </td>
        </tr>
    );
}