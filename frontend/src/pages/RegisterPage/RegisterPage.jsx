import { Col,Row,Container } from "react-bootstrap"
import RegisterForm from "../../components/user/register_form"
// import "./LoginPage.css";
export default function RegisterPage() {
    
    return <Container fluid className="main-container" >
        <Row className="login-form" style={{minHeight:"100vh"}}>
            <Col md="4" className="main-column login-form-column" >
                <RegisterForm/>
            </Col>                
        </Row>
    </Container>
}


