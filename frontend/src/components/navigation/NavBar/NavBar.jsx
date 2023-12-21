import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { Button, Nav, NavDropdown } from 'react-bootstrap';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';

export default function NavBar({userState}) {
    return (
        <Navbar expand="lg" bg='primary' className='bg-body-tetiary'>
            <Container fluid>
                <Navbar.Brand href='/'>NewsRss</Navbar.Brand>
                <Navbar.Collapse id='navbar-colapse-login' className='justify-content-end'>
                    <SelectInfo userState={userState}></SelectInfo>
                </Navbar.Collapse>
                {/* <Navbar.Brand href='/login'>I am home</Navbar.Brand> */}
            </Container>
        </Navbar>
    );
}

function SelectInfo({userState}) {
    const [username,setUsername] = userState;    
    const [isLoading,data,err,logout] = useCustomFetch(logout_api,(data)=>{setUsername("")});
    if (username!=="") {
        return <>
            <Navbar.Text className='m-1'>
                Logged as: {username}
            </Navbar.Text>
            <Button variant='secondary' onClick={logout}>
                Logout
            </Button>
        </>
    }
    return <>
        <Nav.Link href='/login'>Login</Nav.Link>
    </>;
}
