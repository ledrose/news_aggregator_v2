import { Outlet } from "react-router-dom";
import NavBar from "../../components/navigation/NavBar/NavBar";
import InfoPopover from "../../components/navigation/InfoPopover/InfoPopover";
import "./MainLayout.css";
import { Container, Row, Col } from "react-bootstrap";

export default function MainLayout() {
    return (
        <div>
            <InfoPopover/>
            <NavBar/>
            <Container fluid className="main-container">
                <Row className="justify-content-center">
                    <Col md="8" className="main-column">
                        <Outlet/>
                    </Col>                
                </Row>
            </Container>
        </div>
    )
}