import { Col,Row,Container } from "react-bootstrap"
import LoginForm from "../../components/user/login_form"
import "./LoginPage.css";
export default function LoginPage() {
    
    return <Container fluid className="main-container" >
        <Row className="login-form" style={{minHeight:"100vh"}}>
            <Col md="4" className="main-column login-form-column" >
                <LoginForm/>
            </Col>                
        </Row>
    </Container>
}


