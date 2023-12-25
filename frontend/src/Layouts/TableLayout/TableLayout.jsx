import { Container, Row, Col } from "react-bootstrap"
import { useOutletContext } from "react-router-dom"
import "./TableLayout.css";
export default function TableLayout({children}) {
    const headerHeight = useOutletContext();
    return <Container fluid className="main-container" >
        <Row className="table-row" style={{minHeight: window.innerHeight-headerHeight +'px'}}>
            <Col md="8" className="main-column table-column" >
                {children}
            </Col>                
        </Row>
    </Container>
}
