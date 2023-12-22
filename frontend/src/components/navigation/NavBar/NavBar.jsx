import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { Button, Nav, NavDropdown } from 'react-bootstrap';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';
import { useSelector } from 'react-redux';
import { reset, setUser } from '../../../_store/userSlice';
export default function NavBar() {
    return (
        <Navbar expand="lg" bg='primary' className='bg-body-tetiary'>
            <Container fluid>
                <Navbar.Brand href='/'>NewsRss</Navbar.Brand>
                <Navbar.Text>
                    {/* <InfoPopover></InfoPopover> */}
                </Navbar.Text>
                <Navbar.Collapse id='navbar-colapse-login' className='justify-content-end'>
                    <SelectInfo></SelectInfo>
                </Navbar.Collapse>
                {/* <Navbar.Brand href='/login'>I am home</Navbar.Brand> */}
            </Container>
        </Navbar>
    );
}

function SelectInfo() {
    const userInfo = useSelector((state) => state.user);

    const [isLoading,data,err,logout] = useCustomFetch(logout_api,(data)=>{reset()});
    if (userInfo.email!==null) {
        return <>
            <Navbar.Text className='m-1'>
                Logged as: {userInfo.email}
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
