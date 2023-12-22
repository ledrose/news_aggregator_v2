import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"
import usePersistentState from "../../_helpers/UsePersistent";
import { redirect, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import {Button, Form, Spinner} from "react-bootstrap";
import useCustomFetch from "../../_helpers/CustomFetchHook";
export default function LoginForm({userState,errorState}) {
    const navigate = useNavigate();
    const [username,setUsername] = userState;
    const [errorGlobal,setErrorGlobal] = errorState;
    const [isLoading,data,err,sendRequest] = useCustomFetch(login_api,(data)=>{
        setUsername(data.email);
        navigate("/");
    },(err)=>setErrorGlobal(err));
    const onSubmit = (e) => {
        const target = e.target
        e.preventDefault();
        sendRequest(target.email.value,target.password.value)
    }

    return <>
        <Form onSubmit={onSubmit}>
            <Form.Group className="mb-3" controlId="formEmail">
                <Form.Label>Email address</Form.Label>
                <Form.Control name="email" type="email" placeholder="Enter email"></Form.Control>
                <Form.Text className="text-muted">
                    Please, enter email.
                </Form.Text>
            </Form.Group>
            <Form.Group className="mb-3" controlId="formPassword">
                <Form.Label>Password</Form.Label>
                <Form.Control name="password" type="password" placeholder="Password"></Form.Control>
            </Form.Group>
            <Button variant="primary" type="submit">
                Submit
            </Button>
        </Form>
        { isLoading===true &&
            <div>Loading</div>
        }
        { data!==null &&
            <div>{JSON.stringify(data)}</div>
        }
    </>
}
