import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"
import usePersistentState from "../../_helpers/UsePersistent";
import { Link, redirect, useNavigate } from "react-router-dom";
import {Button, Form, Row,Col, Spinner} from "react-bootstrap";
import useCustomFetch from "../../_helpers/CustomFetchHook";
import { reset, setUser } from "../../_store/userSlice";
import { useDispatch } from "react-redux";
import { useState } from "react";
export default function LoginForm() {
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const {register, handleSubmit, formState: {errors}} = useForm();
    const [validated,setValidated] = useState(false);
    const [isLoading,data,err,sendRequest] = useCustomFetch(login_api,(data)=>{
        console.log({email:data.email,role:data.role});
        // reset();
        dispatch(setUser({email:data.email,role:data.role,token:data.token}));
        navigate("/");
    });
    const onSubmit = (data) => {
        sendRequest(data.email,data.password)
    }
    return <>
        <Form onSubmit={handleSubmit(onSubmit)}>
            <Form.Group controlId="formEmail">
                <Form.Label>Почтовый адрес</Form.Label>
                <Form.Control required name="email" type="email" placeholder="example@mail.ru" {...register("email",{required: true, pattern: /^[\w\-\.]+@([\w-]+\.)+[\w-]{2,}$/})}></Form.Control>
                {errors.email && <small>Почта должна быть корректной</small>}
                <div className="mb-3"></div>
            </Form.Group>
            <Form.Group controlId="formPassword">
                <Form.Label>Пароль</Form.Label>
                <Form.Control name="password" type="password" placeholder="Пароль" {...register("password",{required: true})} ></Form.Control>
                {errors.password && <small>Пароль не должен быть пустым</small>}
                <div className="mb-3"></div>
            </Form.Group>
            <Row className="justify-content-between">
                <Col sm="3">
                    <Button variant="primary" type="submit">
                        Войти
                    </Button>
                </Col>
                <Col sm="4">
                    <Link className="btn btn-primary" to={"/register"}>Зарегистрироваться</Link>
                </Col>
            </Row>
        </Form>
    </>
}
