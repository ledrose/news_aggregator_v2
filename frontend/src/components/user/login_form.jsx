import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"
import usePersistentState from "../../_helpers/UsePersistent";
import { Link, redirect, useNavigate } from "react-router-dom";
import {Button, Form, Row,Col, Spinner} from "react-bootstrap";
import useCustomFetch from "../../_helpers/CustomFetchHook";
import { reset, setUser } from "../../_store/userSlice";
import { useDispatch } from "react-redux";
export default function LoginForm() {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const [isLoading,data,err,sendRequest] = useCustomFetch(login_api,(data)=>{
        console.log({email:data.email,role:data.role});
        // reset();
        dispatch(setUser({email:data.email,role:data.role}));
        navigate("/");
    });
    const onSubmit = (e) => {
        const target = e.target;
        e.preventDefault();
        sendRequest(target.email.value,target.password.value)
    }

    return <>
        <Form onSubmit={onSubmit}>
            <Form.Group className="mb-3" controlId="formEmail">
                <Form.Label>Почтовый адрес</Form.Label>
                <Form.Control name="email" type="email" placeholder="example@mail.ru"></Form.Control>
                <Form.Text className="text-muted">
                    Введите почту
                </Form.Text>
            </Form.Group>
            <Form.Group className="mb-3" controlId="formPassword">
                <Form.Label>Пароль</Form.Label>
                <Form.Control name="password" type="password" placeholder="Password"></Form.Control>
            </Form.Group>
            <Row className="justify-content-between">
                <Col sm="2">
                    <Button variant="primary" type="submit">
                        Войти
                    </Button>
                </Col>
                <Col sm="4">
                    {/* <Button variant="primary">Зарегистрироваться</Button> */}
                    <Link to={"/register"} className="btn btn-primary">Зарегистрироваться</Link>
                </Col>
            </Row>
        </Form>
        { isLoading===true &&
            <div>Загрузка</div>
        }
    </>
}
