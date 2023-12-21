import Container from 'react-bootstrap/Container';
import Navbar from 'react-bootstrap/Navbar';
import { logout_api } from '../../backend_api/login';
import usePersistentState from '../../../_helpers/UsePersistent';
import useCustomFetch from '../../../_helpers/CustomFetchHook';

export default function NavBar({userState}) {
    return (
        <Navbar expand="lg">
            <Container fluid className='Navbar' >
                <SelectInfo userState={userState}></SelectInfo>
                {/* <Navbar.Brand href='/login'>I am home</Navbar.Brand> */}
            </Container>
        </Navbar>
    );
}

function SelectInfo({userState}) {
    const [username,setUsername] = userState;    
    const [isLoading,data,err,sendRequest] = useCustomFetch(logout_api,(data)=>{setUsername("")});
    const logout = () => {sendRequest()};
    // console.log(username);
    // const is_logged = (username!="")?true:false;
    if (username!=="") {
        return <div>
            <p>Current_user: {username}</p>
            <button onClick={logout}>Logout</button>
        </div>
    }
    return (<a href='/login'>You must be logged</a>);
}
