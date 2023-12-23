import { Button, Container, Table } from "react-bootstrap"
import useCustomFetch from "../_helpers/CustomFetchHook"
import { get_sources_api } from "../components/backend_api/admin";
import { useEffect, useState } from "react";
export default function ListSourcesPage() {
    const amount_on_page = 15;
    const [firstId,setFirstId] = useState(0);
    const [sources,setSources] = useState([]);
    const [isLoading,data,error,sendRequest] = useCustomFetch(get_sources_api,(data) => {
        console.log(data);
        if (data.length!=0) {
            setFirstId(data[0].id)
            setSources(data);
        }
    });
    useEffect(() => {
        sendRequest(0,amount_on_page);
    },[])
    const nextPage = () => {
        sendRequest(firstId-amount_on_page,amount_on_page);
    }
    const prevPage = () => {
        sendRequest(firstId+amount_on_page,amount_on_page);
    }
    const nextDis = amount_on_page>sources.length;
    const prevDis = firstId===1;
    return (
        <Container>
            <Table striped bordered hover>
                <thead>
                    <tr>
                        <th>#</th>
                        <th>Название</th>
                        <th>Тип</th>
                        <th>Ссылка</th>
                    </tr>
                </thead>
                <tbody>
                    {data && data?.map((el) => (
                        <TableRow data={el}></TableRow>
                    ))}
                </tbody>
            </Table>
            <Button disabled={prevDis} onClick={nextPage}>Prev</Button>
            <Button disabled={nextDis} onClick={prevPage}>Next</Button>
        </Container>
    )
}

function TableRow(data) {
    return (
        <tr>
            <td>{data.id}</td>
            <td>{data.name}</td>
            <td>{data.source_type || ""}</td>
            <td>{data.link || ""}</td>
        </tr>
    );
}