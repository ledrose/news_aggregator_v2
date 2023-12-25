import { Container,Row,Col } from "react-bootstrap"
export default function ScrollLayout({children}) {
    return <Container fluid className="main-container" style={{minHeight:"100vh"}}>
        <Row className="justify-content-center" style={{minHeight:"100vh"}}>
            <Col md="8" className="main-column">
                {children}
            </Col>                
        </Row>
    </Container>
}