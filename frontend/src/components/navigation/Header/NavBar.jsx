import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import {Link} from 'react-router-dom';

export default function NavBar() {
    const is_logged = false;
    const is_admin = false;
    return (
        <Navbar expand="lg">
            <Container fluid className='Navbar' >
                <Navbar.Brand href='/login'>I am home</Navbar.Brand>
            </Container>
        </Navbar>
    );
}