import { Container, Row, Col } from "react-bootstrap"
import { useOutletContext } from "react-router-dom"
export default function StaticLayout({children}) {
    const context = useOutletContext();
    return <Container fluid className="main-container" >
        <Row className="login-form" style={{minHeight: window.innerHeight-context.headerHeight +'px'}}>
            <Col md="4" className="main-column login-form-column" >
                {children}
            </Col>                
        </Row>
    </Container>
}
